import './Chat.css';

import React, { useState } from 'react';

import OpenChat from './OpenChat';

function Chat() {

  const [username, setUsername] = useState('');
  const [showChat, setShowChat] = useState(false);

  let joinChat = function(e) {
    setShowChat(() => true);
  }

  return (
    <div className="Chat">
      <header className="Chat-header">
        <input id="username" className="user-name" type="text" placeholder="username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        />
        <button
          id="join-chat"
          type="button"
          onClick={joinChat}
        >Join Chat</button>

        {showChat && <OpenChat username={username}></OpenChat>}
      </header>
    </div>
  );
}

export default Chat;