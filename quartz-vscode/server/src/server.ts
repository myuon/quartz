import {
  CompletionItemKind,
  createConnection,
  Diagnostic,
  ProposedFeatures,
  TextDocuments,
  TextDocumentSyncKind,
} from "vscode-languageserver/node";
import { TextDocument } from "vscode-languageserver-textdocument";
import { exec } from "child_process";
import * as path from "path";
import * as util from "util";

const execAsync = util.promisify(exec);

const connection = createConnection(ProposedFeatures.all);

const documents = new TextDocuments(TextDocument);

connection.onInitialize(() => {
  return {
    capabilities: {
      textDocumentSync: TextDocumentSyncKind.Full,
      hoverProvider: true,
      definitionProvider: true,
      completionProvider: {
        triggerCharacters: ["."],
      },
    },
  };
});

connection.onInitialized(() => {
  connection.console.log("Initialized!");
});

let currentContent = "";

documents.onDidChangeContent(async (change) => {
  currentContent = change.document.getText();

  const diagnostics: Diagnostic[] = [];

  const file = change.document.uri.replace("file://", "");

  const cargo = await execAsync(
    `cargo run --manifest-path ${path.join(
      file,
      "..",
      "..",
      "Cargo.toml"
    )} -- check ${file} --project ${path.join(file, "..", "..")}`
  );
  if (cargo.stdout) {
    const errors: {
      message: string;
      start: [number, number];
      end: [number, number];
    }[] = JSON.parse(cargo.stdout);

    errors.forEach((error) => {
      diagnostics.push({
        severity: 1,
        range: {
          start: { line: error.start[0], character: error.start[1] },
          end: { line: error.end[0], character: error.end[1] },
        },
        message: error.message,
        source: "ex",
      });
    });
  }

  connection.sendDiagnostics({ uri: change.document.uri, diagnostics });
});

connection.onHover(async (params) => {
  const file = params.textDocument.uri.replace("file://", "");
  console.log(params);

  const command = `cargo run --manifest-path ${path.join(
    file,
    "..",
    "..",
    "Cargo.toml"
  )} -- check-type ${file} --project ${path.join(file, "..", "..")} --line ${
    params.position.line
  } --column ${params.position.character}`;
  console.log(command);
  const cargo = await execAsync(command);
  if (cargo.stdout) {
    return { contents: cargo.stdout };
  }
});

connection.onDefinition(async (params) => {
  const file = params.textDocument.uri.replace("file://", "");
  console.log(params);

  const cargo = await execAsync(
    `cargo run --manifest-path ${path.join(
      file,
      "..",
      "..",
      "Cargo.toml"
    )} -- go-to-def ${file} --project ${path.join(file, "..", "..")} --line ${
      params.position.line
    } --column ${params.position.character}`
  );
  if (cargo.stdout) {
    const result = JSON.parse(cargo.stdout) as {
      file: string;
      start: {
        line: number;
        column: number;
      };
      end: {
        line: number;
        column: number;
      };
    };

    return [
      {
        uri: `file://${result.file}`,
        range: {
          start: {
            line: result.start.line,
            character: result.start.column,
          },
          end: {
            line: result.end.line,
            character: result.end.column,
          },
        },
      },
    ];
  }

  return null;
});

connection.onCompletion(async (params) => {
  console.log("completion", params);
  const file = params.textDocument.uri.replace("file://", "");

  const command = `cargo run --manifest-path ${path.join(
    file,
    "..",
    "..",
    "Cargo.toml"
  )} -- completion ${file} --project ${path.join(file, "..", "..")} --line ${
    params.position.line
  } --column ${
    params.position.character
  } --stdin <<EOF\n${currentContent}\nEOF\n`;
  console.log(command);

  const cargo = await execAsync(command);
  if (cargo.stdout) {
    const result = JSON.parse(cargo.stdout) as {
      items: {
        kind: "function";
        label: string;
        detail: string;
      }[];
    };

    return result.items.map((item) => ({
      label: item.label,
      kind:
        item.kind === "function"
          ? CompletionItemKind.Function
          : CompletionItemKind.Text,
      detail: item.detail,
    }));
  }

  return undefined;
});

documents.listen(connection);

connection.listen();
