import './Chat.css';

import { useState } from 'react';

import OpenChat from './OpenChat';

function Chat() {

  const [username, setUsername] = useState('');
  const [userType, setUserType] = useState('CUSTOMER');
  const [showChat, setShowChat] = useState(false);

  let joinChat = function() {
    setShowChat(() => true);
  }

  return (
    <div className="Chat">
      <header className="Chat-header">
        <input id="username" className="user-name" type="text" placeholder="username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        />
        <input id="customer" name="user_type" type="radio" value="CUSTOMER"
               onChange={(e) => setUserType(e.target.value)}
        />
        <label htmlFor="customer">Customer</label><br/>
        <input id="advisor" name="user_type" type="radio" value="ADVISOR"
               onChange={(e) => setUserType(e.target.value)}
        />
        <label htmlFor="advisor">Advisor</label><br/>
        <button
          id="join-chat"
          type="button"
          onClick={joinChat}
        >Join Chat</button>

        {showChat && <OpenChat username={username} user_type={userType}></OpenChat>}
      </header>
    </div>
  );
}

export default Chat;