import React, { useEffect } from "react";

export default function Popup() {
  useEffect(() => {
    initDragElement();
    initResizeElement();
  }, []);

  function initDragElement() {
    let pos1 = 0,
      pos2 = 0,
      pos3 = 0,
      pos4 = 0;
    const popups = document.getElementsByClassName("popup");
    let elmnt = null;
    let currentZIndex = 100;

    for (let i = 0; i < popups.length; i++) {
      const popup = popups[i];
      const header = getHeader(popup);

      popup.onmousedown = function () {
        this.style.zIndex = `${++currentZIndex}`;
      };

      if (header) {
        header.parentPopup = popup;
        header.onmousedown = dragMouseDown;
      }
    }

    function dragMouseDown(e) {
      elmnt = this.parentPopup;
      elmnt.style.zIndex = `${++currentZIndex}`;

      e = e || window.event;
      pos3 = e.clientX;
      pos4 = e.clientY;
      document.onmouseup = closeDragElement;
      document.onmousemove = elementDrag;
    }

    function elementDrag(e) {
      if (!elmnt) {
        return;
      }

      e = e || window.event;
      pos1 = pos3 - e.clientX;
      pos2 = pos4 - e.clientY;
      pos3 = e.clientX;
      pos4 = e.clientY;
      elmnt.style.top = `${elmnt.offsetTop - pos2}px`;
      elmnt.style.left = `${elmnt.offsetLeft - pos1}px`;
    }

    function closeDragElement() {
      document.onmouseup = null;
      document.onmousemove = null;
    }

    function getHeader(element) {
      const headerItems = element.getElementsByClassName("popup-header");

      if (headerItems.length === 1) {
        return headerItems[0];
      }

      return null;
    }
  }

  function initResizeElement() {
    const popups = document.getElementsByClassName("popup");
    let element = null;
    let startX, startY, startWidth, startHeight;

    for (let i = 0; i < popups.length; i++) {
      const p = popups[i];

      const right = document.createElement("div");
      right.className = "resizer-right";
      p.appendChild(right);
      right.addEventListener("mousedown", initDrag, false);
      right.parentPopup = p;

      const bottom = document.createElement("div");
      bottom.className = "resizer-bottom";
      p.appendChild(bottom);
      bottom.addEventListener("mousedown", initDrag, false);
      bottom.parentPopup = p;

      const both = document.createElement("div");
      both.className = "resizer-both";
      p.appendChild(both);
      both.addEventListener("mousedown", initDrag, false);
      both.parentPopup = p;
    }

    function initDrag(e) {
      element = this.parentPopup;

      startX = e.clientX;
      startY = e.clientY;
      startWidth = parseInt(
        document.defaultView.getComputedStyle(element).width,
        10
      );
      startHeight = parseInt(
        document.defaultView.getComputedStyle(element).height,
        10
      );
      document.documentElement.addEventListener("mousemove", doDrag, false);
      document.documentElement.addEventListener("mouseup", stopDrag, false);
    }

    function doDrag(e) {
      element.style.width = `${startWidth + e.clientX - startX}px`;
      element.style.height = `${startHeight + e.clientY - startY}px`;
    }

    function stopDrag() {
      document.documentElement.removeEventListener(
        "mousemove",
        doDrag,
        false
      );
      document.documentElement.removeEventListener(
        "mouseup",
        stopDrag,
        false
      );
    }
  }

  return (
    <div className="popup">
      <div className="popup-header">new window</div>
    </div>
  );
}
