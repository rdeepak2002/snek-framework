export const makeDraggable = (dragElement, cb) => {
    if (document.getElementById(dragElement.id)) {
        document.getElementById(dragElement.id).onmousedown = dragMouseDown;
    } else {
        dragElement.onmousedown = dragMouseDown;
    }

    function dragMouseDown(e) {
        e = e || window.event;
        e.preventDefault();
        document.onmouseup = closeDragElement;
        document.onmousemove = elementDrag;
    }

    function elementDrag(e) {
        e = e || window.event;
        e.preventDefault();
        cb(e);
    }

    function closeDragElement() {
        document.onmouseup = null;
        document.onmousemove = null;
    }
}
