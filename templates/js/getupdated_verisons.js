function textAreaAdjust(element) {
    element.style.height = "1px";
    element.style.height = (25 + element.scrollHeight) + "px";
}
function textAreaAdjust(element) {
    element.style.height = "1px";
    element.style.height = (25 + element.scrollHeight) + "px";
}
var receive_uri = "ws://127.0.0.1:8000/rx";
function init() {
    log = document.getElementById("listbox");
    testWebSocket();


}

function testWebSocket() {
    websocket = new WebSocket(receive_uri);
    websocket.onopen = onOpen;
    websocket.onclose = onClose;
    websocket.onmessage = onMessage;
}

function onOpen(evt) {
    console.log("CONNECTED TO WEBSOCKET " + evt.data);
}

function onClose(evt) {
    console.log("Websocket DISCONNECTED " + evt.data);
    document.getElementById("loader").ariaBusy = false;
    // var pre = document.getElementById("listbox").innerHTML+="<div></div>";
}

function onMessage(evt) {
    writeLog(evt.data);
}


function writeLog(message) {
    console.log(message);
    var pre = document.getElementById("listbox");
    var html =
        "<li>\
            <label onclick=\"setradio(this)\" for=\"small\">\
                <input type=\"radio\" name=\"version\" value=\"{message}\"> \
                {message} \
            </label> \
        </li>".replaceAll("\n", "").replaceAll("{message}", message);
    pre.innerHTML += html;

}

function setradio(msg) {
    document.getElementById("loader").innerText = msg.innerText;
}

window.addEventListener("load", init, false);

var servers = document.querySelectorAll('[id=server_name]');
var server_status = document.querySelectorAll('[id=server_status]');

function query_status() {
    var websockets = []
    for (let i = 0; i < servers.length; i++) {
        let text = servers[i].innerHTML.split(' - ')[0];
        var receive_uri = "wss://console.natimerry.com/status/";
        receive_uri += text;
        websockets.push(new WebSocket(receive_uri));
    }
    for (let i = 0; i < websockets.length; i++) {
        // console.log(websockers[i]);
        // websockets[i].onopen = onOpen;
        // websockets[i].onclose = onClose;
        websockets[i].onmessage = function update(evt) {
            let msg = evt.data;
            let arr = server_status[i].innerHTML.split('-');
            server_status[i].innerHTML = arr[0] + "- " + msg;
        };
    }
}

function startServer(index) {
    var ws_uri = "ws://127.0.0.1:8000/start/" + servers[index].innerHTML;
    console.log(ws_uri);

    var ws = new WebSocket(ws_uri);

    ws.onopen = (event) =>{
        console.log("Starting server");
    }
    ws.onclose = (event) =>{
        console.log("Started server");
    }

}
function stopServer(index) {
    var ws_uri = "ws://127.0.0.1:8000/stop/" + servers[index].innerHTML;
    console.log(ws_uri);

    var ws = new WebSocket(ws_uri);

    ws.onopen = (event) =>{
        console.log("Stopping server");
    }
    ws.onclose = (event) =>{
        console.log("Stopped server");
    }

}


window.addEventListener("load", init, false);
query_status();
// setInterval(query_status, 5000);
