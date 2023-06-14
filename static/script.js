let button = document.getElementById('.button');

button.addEventListener("click", function (e) {
    e.preventDefault();

    fetch("/api/start")
        .then((response) => {
          return response.json();
        })
        .then((data) => window.open(data.url));
  });
  
function winamp(){
  const app = document.getElementById("app")
  const webamp = new Webamp();
  webamp.appendTracks([
    //{url: 'demo.mp3'},
  ]);
  webamp.renderWhenReady(app);
  console.log('rendered webamp!');
}