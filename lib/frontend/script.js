window.onload = function () {
  vm_renderer.init(document.getElementById('row'))
}

let vm_renderer = {
  kill: false,
  fetch: async function (url) {
    const response = await fetch(url)
    if (!response.ok) {
      alert("unexpected error has occurred: " + response.status + " " + response.statusText + ". is the server offline? ");
      throw new Error("unexpected error has occurred: " + response.status + " " + response.statusText + ". is the server offline? ");
    }
    const vmids = await response.json();
    return vmids;
  },
  make_card: function (slot, rows) {
    // console.log(slot);
    var padding = document.createElement('div');
    var card = document.createElement('div');
    var open_vnc_on_click = document.createElement('from');
    var card_header = document.createElement('div');
    var image = document.createElement('img');
    var card_header = document.createElement('div');

    card_header.classname = "card_header";
    if (slot.runing) {
      card_header.innerText = "VM " + (slot.vmid + 1) + " (runing)";
    } else {
      card_header.innerText = "VM " + (slot.vmid + 1) + " " + slot.name + " (start)";
    }
    card_header.setAttribute('class', 'card-header')

    image.setAttribute('src', 'placeholder.png')
    image.setAttribute('width', '310')
    image.setAttribute('height', '235')

    card.classname = "card";
    card.setAttribute('class', 'card')
    card.appendChild(card_header)
    card.appendChild(image)

    open_vnc_on_click.setAttribute('onclick', 'vm_renderer.onclick('+ slot.vmid +')')
    open_vnc_on_click.appendChild(card)
    
    padding.setAttribute('class', 'padding')
    padding.appendChild(open_vnc_on_click)
    rows.appendChild(padding)
  },
  onclick: function (vmid) {
    if (this.kill) {
      this.fetch("/api/stop?number=" + vmid).then((data) => console.log("popup status: " + data));
      this.kill = false;
    } else {
      this.fetch("/api/start?number=" + vmid).then((data) => {console.log("popup status: " + data)});
      popup = window.open('/noVNC/vnc.html?path=api/stream/' + vmid, 'popup', 'width=700,height=600,status=no,scrollbars=no,resizable=yes')
      popup.focus()
    }
  },
  init: function (rows) {
    this.fetch("/api/statistics").then((data) => data.vm_list.forEach((slot) => this.make_card(slot, rows)));
  }
}