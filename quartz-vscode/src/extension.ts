import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  try {
    console.log("Activating extension: " + context.extensionPath);

    const serverModule = context.asAbsolutePath("server/out/server.js");

    const serverOptions: ServerOptions = {
      run: {
        module: serverModule,
        transport: TransportKind.ipc,
      },
      debug: {
        module: serverModule,
        transport: TransportKind.ipc,
      },
    };

    const clientOptions: LanguageClientOptions = {
      documentSelector: [{ scheme: "file", language: "quartz" }],
    };
    client = new LanguageClient(
      "quartz-mode",
      "Quartz Language Server",
      serverOptions,
      clientOptions
    );
    context.subscriptions.push(client.start());
  } catch (e) {
    vscode.window.showErrorMessage("Error activating extension: " + e);
  }
}

// This method is called when your extension is deactivated
export function deactivate() {
  if (client) {
    return client.stop();
  }
}
