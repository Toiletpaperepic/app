var wsUri = "/api/console";
var log;

function init() {
    log = document.getElementById("log");
    form = document.getElementsByTagName("form")[0];
    message = document.getElementById("message");

    testWebSocket();

    form.addEventListener("submit", (e) => {
        e.preventDefault();
        if (message.value !== "") {
            if (message.value == "reconnected") {
                testWebSocket()
            } else {
                sendMessage(message.value);
            }
            message.value = "";
        }
    });
}

function testWebSocket() {
    if (window.location.protocol == "https:") 
        var protocol = "wss:";
    else 
        var protocol = "ws:";
    var url = protocol  + window.location.host + wsUri;
    console.log(url)
    websocket = new WebSocket(url);
    websocket.onopen = onOpen;
    websocket.onclose = onClose;
    websocket.onmessage = onMessage;
    websocket.onerror = onError;
}

function onOpen(evt) {
    writeLog("CONNECTED");
}

function onClose(evt) {
    writeLog("Websocket DISCONNECTED: type 'reconnected' to reconnected");
}

function onMessage(evt) {
    writeLog('<span style="color: blue;"> Server: ' + evt.data + '</span>');
}

function onError(evt) {
    writeLog('<span style="color: red;">ERROR:</span> ' + evt.data);
}

function sendMessage(message) {
    writeLog("SENT: " + message);
    websocket.send(message);
}

function writeLog(message) {
    var pre = document.createElement("p");
    pre.innerHTML = message;
    log.prepend(pre);
}

window.addEventListener("load", init, false);