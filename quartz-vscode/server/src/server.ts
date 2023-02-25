import {
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
    },
  };
});

connection.onInitialized(() => {
  connection.console.log("Initialized!");
});

documents.onDidChangeContent(async (change) => {
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

documents.listen(connection);

connection.listen();
