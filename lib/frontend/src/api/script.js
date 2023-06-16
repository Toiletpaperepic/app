export default function RunButton() {
    let msg = "start";
    const api_data = {
        slotSize: 10 
      };

    function handleSubmit(event) {
        event.preventDefault()

        fetch("/api/start")
            .then((response) => {
                if (!response.ok) {
                    alert("No found.");
                    throw new Error("No found.");
                }
            return response.json();
            })
            .then((data) => window.open(data.url));
        msg = "runing";
    }

  return (
    <form onClick={handleSubmit}>
        <button type="submit">{msg}</button>
        <input className="search-bar" placeholder={"Choose a vm Between 0 to " + api_data.slotSize}></input>
    </form>
    );
  }