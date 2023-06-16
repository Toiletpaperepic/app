export default function RunButton() {
    let msg = "start";

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
            .then((data) => window.open(data.url))
    }

  return (
    <form onClick={handleSubmit}>
        <button type="submit">{msg}</button>
    </form>
    );
  }