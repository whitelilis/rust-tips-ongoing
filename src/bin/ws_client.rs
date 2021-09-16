use websocket::{ClientBuilder, Message};
use websocket::client::sync::Client;
use std::net::TcpStream;


///
/// This code snippet is for test websocket and jedi-language-server for python

fn main(){
    let uri = "ws://127.0.0.1:7777/halo-language-server";
    let mut client = ClientBuilder::new(uri)
        .unwrap()
        .connect_insecure()
        .unwrap();


    let messages = [
r#"{"jsonrpc":"2.0","id":0,"method":"initialize","params":{"processId":88985,"rootPath":"/Users/liuzhe/src/pythobn","rootUri":"file:///Users/liuzhe/src/pythobn","capabilities":{"workspace":{"applyEdit":true,"workspaceEdit":{"documentChanges":true,"resourceOperations":["create","rename","delete"],"failureHandling":"textOnlyTransactional"},"didChangeConfiguration":{"dynamicRegistration":true},"didChangeWatchedFiles":{"dynamicRegistration":true},"symbol":{"dynamicRegistration":true,"symbolKind":{"valueSet":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26]},"tagSupport":{"valueSet":[1]}},"codeLens":{"refreshSupport":true},"executeCommand":{"dynamicRegistration":true},"configuration":true,"semanticTokens":{"refreshSupport":true},"fileOperations":{"dynamicRegistration":true,"didCreate":true,"didRename":true,"didDelete":true,"willCreate":true,"willRename":true,"willDelete":true},"workspaceFolders":true},"textDocument":{"publishDiagnostics":{"relatedInformation":true,"versionSupport":false,"tagSupport":{"valueSet":[1,2]}},"synchronization":{"dynamicRegistration":true,"willSave":true,"willSaveWaitUntil":true,"didSave":true},"completion":{"dynamicRegistration":true,"contextSupport":true,"completionItem":{"snippetSupport":true,"commitCharactersSupport":true,"documentationFormat":["markdown","plaintext"],"deprecatedSupport":true,"preselectSupport":true,"insertReplaceSupport":true,"tagSupport":{"valueSet":[1]},"resolveSupport":{"properties":["documentation","detail","additionalTextEdits"]},"insertTextModeSupport":{"valueSet":[1,2]}},"completionItemKind":{"valueSet":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25]}},"hover":{"dynamicRegistration":true,"contentFormat":["markdown","plaintext"]},"signatureHelp":{"dynamicRegistration":true,"contextSupport":true,"signatureInformation":{"documentationFormat":["markdown","plaintext"],"activeParameterSupport":true,"parameterInformation":{"labelOffsetSupport":true}}},"definition":{"dynamicRegistration":true},"references":{"dynamicRegistration":true},"documentHighlight":{"dynamicRegistration":true},"documentSymbol":{"dynamicRegistration":true,"symbolKind":{"valueSet":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26]},"hierarchicalDocumentSymbolSupport":true,"tagSupport":{"valueSet":[1]}},"codeAction":{"dynamicRegistration":true,"isPreferredSupport":true,"disabledSupport":true,"dataSupport":true,"honorsChangeAnnotations":false,"resolveSupport":{"properties":["edit"]},"codeActionLiteralSupport":{"codeActionKind":{"valueSet":["","quickfix","refactor","refactor.extract","refactor.inline","refactor.rewrite","source","source.organizeImports"]}}},"codeLens":{"dynamicRegistration":true},"formatting":{"dynamicRegistration":true},"rangeFormatting":{"dynamicRegistration":true},"onTypeFormatting":{"dynamicRegistration":true},"rename":{"dynamicRegistration":true,"prepareSupport":true},"documentLink":{"dynamicRegistration":true,"tooltipSupport":true},"typeDefinition":{"dynamicRegistration":true},"implementation":{"dynamicRegistration":true},"declaration":{"dynamicRegistration":true},"colorProvider":{"dynamicRegistration":true},"foldingRange":{"dynamicRegistration":true,"rangeLimit":5000,"lineFoldingOnly":true},"selectionRange":{"dynamicRegistration":true},"callHierarchy":{"dynamicRegistration":true},"semanticTokens":{"dynamicRegistration":true,"tokenTypes":["namespace","type","class","enum","interface","struct","typeParameter","parameter","variable","property","enumMember","event","function","method","macro","keyword","modifier","comment","string","number","regexp","operator"],"tokenModifiers":["declaration","definition","readonly","static","deprecated","abstract","async","modification","documentation","defaultLibrary"],"formats":["relative"],"requests":{"range":true,"full":{"delta":true}},"multilineTokenSupport":false,"overlappingTokenSupport":false},"linkedEditingRange":{"dynamicRegistration":true}},"window":{"showMessage":{"messageActionItem":{"additionalPropertiesSupport":false}},"showDocument":{"support":false},"workDoneProgress":true},"general":{"regularExpressions":{"engine":"ECMAScript","version":"ES2020"},"markdown":{"parser":"marked","version":"1.1.0"}}},"initializationOptions":{"enable":true,"startupMessage":false,"trace":{"server":"verbose"},"jediSettings":{"autoImportModules":["numpy","pandas","torch"],"caseInsensitiveCompletion":true},"executable":{"args":["--log-file","/tmp/auto_self.log","-v"],"command":"jedi-language-server"},"codeAction":{"nameExtractFunction":"jls_extract_def","nameExtractVariable":"jls_extract_var"},"completion":{"disableSnippets":false,"resolveEagerly":false},"diagnostics":{"enable":true,"didOpen":true,"didChange":true,"didSave":true},"hover":{"enable":true,"disable":{"class":{"all":false,"names":[],"fullNames":[]},"function":{"all":false,"names":[],"fullNames":[]},"instance":{"all":false,"names":[],"fullNames":[]},"keyword":{"all":false,"names":[],"fullNames":[]},"module":{"all":false,"names":[],"fullNames":[]},"param":{"all":false,"names":[],"fullNames":[]},"path":{"all":false,"names":[],"fullNames":[]},"property":{"all":false,"names":[],"fullNames":[]},"statement":{"all":false,"names":[],"fullNames":[]}}},"workspace":{"extraPaths":[],"symbols":{"maxSymbols":20,"ignoreFolders":[".nox",".tox",".venv","__pycache__","venv"]}},"markupKindPreferred":"plaintext"},"trace":"verbose","workspaceFolders":[{"uri":"file:///Users/liuzhe/src/pythobn","name":"pythobn"}],"locale":"zh_CN","clientInfo":{"name":"coc.nvim","version":"0.0.80"}}}"#,
//b'Content-Length: 52\r\n\r\n{"jsonrpc":"2.0","method":"initialized","params":{}}',
r#"{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":"file:///Users/liuzhe/src/pythobn/kk.py","languageId":"python","version":1,"text":"\\n"}}}"#,
r#"{"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"version":2,"uri":"file:///Users/liuzhe/src/pythobn/kk.py"},"contentChanges":[{"range":{"start":{"line":0,"character":0},"end":{"line":0,"character":0}},"rangeLength":0,"text":"i"}]}}"#,
r#"{"jsonrpc":"2.0","id":1,"method":"textDocument/completion","params":{"textDocument":{"uri":"file:///Users/liuzhe/src/pythobn/kk.py"},"position":{"line":0,"character":1},"context":{"triggerKind":1}}}"#,
r#"{"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"version":3,"uri":"file:///Users/liuzhe/src/pythobn/kk.py"},"contentChanges":[{"range":{"start":{"line":0,"character":1},"end":{"line":0,"character":1}},"rangeLength":0,"text":"m"}]}}"#,
r#"{"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"version":4,"uri":"file:///Users/liuzhe/src/pythobn/kk.py"},"contentChanges":[{"range":{"start":{"line":0,"character":2},"end":{"line":0,"character":2}},"rangeLength":0,"text":"port "}]}}"#,
r#"{"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"version":5,"uri":"file:///Users/liuzhe/src/pythobn/kk.py"},"contentChanges":[{"range":{"start":{"line":0,"character":7},"end":{"line":0,"character":7}},"rangeLength":0,"text":"p"}]}}"#,
r#"{"jsonrpc":"2.0","id":2,"method":"textDocument/completion","params":{"textDocument":{"uri":"file:///Users/liuzhe/src/pythobn/kk.py"},"position":{"line":0,"character":8},"context":{"triggerKind":1}}}"#,
r#"{"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"version":6,"uri":"file:///Users/liuzhe/src/pythobn/kk.py"},"contentChanges":[{"range":{"start":{"line":0,"character":8},"end":{"line":0,"character":8}},"rangeLength":0,"text":"i"}]}}"#,
];



    for message in messages {
        println!(">>>>>>>>>>>>>>>>>>>>             >>>>>>>>>>>>>>>>>");
        // then send
        println!("{}", message);
        let m = Message::text(String::from(message));
        client.send_message(&m).unwrap();
        handle_recv(&mut client);
    }
}

fn handle_recv(client: &mut Client<TcpStream>) {
    println!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
    match client.recv_message() {
        Ok(m) => {
            println!("{:?}", m);
        }
        Err(e) => {
            println!("Send Loop: {:?}", e);
            return;
        }
    };
}