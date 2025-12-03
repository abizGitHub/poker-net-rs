
async function init() {

    let rustApp = null
    try {
        rustApp = await import('../pkg')
    } catch (e) {
        console.log(e)
        return;
    }
    console.log(rustApp)

    const logBox = document.getElementById("log");
    const canvas = document.getElementById("canvas");
    rustApp.set_canvas(canvas);
    
    
    // List of events you want to capture
    const eventsToLog = [
        "click", //"mousemove",
        "keydown", "keyup", "scroll", "input",
        "mousedown", "mouseup", "mouseenter", "mouseleave", "touchstart",
        "touchend"
    ];

    function log(event) {
        let resp = rustApp.on_call(event.type);        
        if (resp != undefined) {
            const msg = `${new Date().toLocaleTimeString()} → ${resp.map(a => a + "<br>")}`;
            logBox.innerHTML += msg + "<br>";
            logBox.scrollTop = logBox.scrollHeight;
        }
        console.log(event.type)
    }

    // Attach event listeners to entire page
    eventsToLog.forEach(eventType => {
        document.addEventListener(eventType, log);
    });

    // Also log button-specific events
    const button = document.getElementById("Deal");
    button.addEventListener("click", () => {
        log({ type: "deal" });
    });
}

init()