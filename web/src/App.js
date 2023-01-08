import logo from './logo.svg';
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <input id="username" className="user-name" type="text" placeholder="username"/>
        <button id="join-chat" type="button">Join Chat</button>
        <textarea id="chat" className="chat-area" cols="30" rows="10"></textarea>
        <input id="input" className="new-message-input" type="text" placeholder="chat"/>
      </header>
    </div>
  );
}

export default App;
