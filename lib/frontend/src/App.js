import './windows/script.js';
import './windows/style.css'
import RunButton from './api/script.js';
import './App.css';

function App() {
  const api_data = {
    slotSize: 10 
  };

  return (
    <div className="app">
      <nav>
        <form id=".button">
          <input className="search-bar" placeholder={"Choose a vm Between 0 to " + api_data.slotSize}></input>
          <RunButton />
        </form>
      </nav>
    </div>
  );
}

export default App;