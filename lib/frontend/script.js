// let button = document.getElementById('.button');

// button.addEventListener("click", function (e) {
//   e.preventDefault();

//   fetch("/api/start")
//     .then((response) => {
//       return response.json();
//     })
//     then((data) => window.open(data.url));
// });

window.onload = function () {
  let rows = document.getElementById('row');
  vm_renderer.fetch_runing_vms(rows)
}

let vm_renderer = {
  fetch_runing_vms: function (rows) {
    let server_data = fetch("/api/statistics")
      .then((response) => {
        if (!response.ok) {
          alert("unexpected error has occurred: " + response.text + " is the server offline? ");
          throw new Error("unexpected error has occurred: " + response.text + " is the server offline? ");
        }
          return response.json();
        })
        .then((data) => data.vm_list.forEach((slot) => this.make_card(slot, rows)));
  },
  make_card: function (slot, rows) {
    console.log(slot);
    var padding = document.createElement('div');
    var card = document.createElement('div');
    var open_vnc_on_click = document.createElement('from');
    var card_header = document.createElement('div');
    var image = document.createElement('img');

    image.setAttribute('src', 'vm.jpg')
    image.setAttribute('width', '310')
    image.setAttribute('height', '235')

    card_header.classname = "card_header";
    card_header.innerText = "VM " + slot.vmid;
    card_header.setAttribute('class', 'card-header')

    card.classname = "card";
    card.setAttribute('class', 'card')
    card.appendChild(card_header)
    card.appendChild(image)

    open_vnc_on_click.setAttribute('onclick', 'vm_renderer.url(' + '"' + slot.start + '"' + ')')
    open_vnc_on_click.appendChild(card)
    
    padding.setAttribute('class', 'padding')
    padding.appendChild(open_vnc_on_click)
    rows.appendChild(padding)
  },
  url: function (url) {
    fetch(url)
      .then((response) => {
        if (!response.ok) {
          alert("unexpected error has occurred: " + response.text + " is the server offline? ");
          throw new Error("unexpected error has occurred: " + response.text + " is the server offline? ");
        }
          return response.json();
        })
        .then((data) => popup = window.open(data.url, 'popup', 'width=700,height=600,status=no,scrollbars=no,resizable=yes'));
    
    popup.focus();
  }
}
  
function winamp(){
  const app = document.getElementById("app")
  const webamp = new Webamp();
  webamp.appendTracks([
    //{url: 'demo.mp3'},
  ]);
  webamp.renderWhenReady(app);
  console.log('rendered webamp!');
}