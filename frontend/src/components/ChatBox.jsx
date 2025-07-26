// src/components/ChatBox.jsx

import React, { useState, useEffect, useRef } from 'react';
import useWebSocket, { ReadyState } from 'react-use-websocket';
import * as api from '../api';
import { useAuth } from '../context/AuthContext'; // 引入 useAuth

function ChatBox({ rfqId }) {
    const { user } = useAuth(); // 获取当前用户信息
    const [messageHistory, setMessageHistory] = useState([]);
    const [message, setMessage] = useState('');
    const chatBoxRef = useRef(null);

    // 动态构建 WebSocket URL，并附上token
    const [socketUrl, setSocketUrl] = useState(null);
    useEffect(() => {
        const token = localStorage.getItem('token');
        if (token) {
            const url = `ws://127.0.0.1:8080/ws/chat/${rfqId}?token=${token}`;
            console.log("Connecting to WebSocket at:", url);
            setSocketUrl(url);
        }
    }, [rfqId]);

    // 使用 useWebSocket hook
    const { sendMessage, lastMessage, readyState } = useWebSocket(socketUrl, {
        shouldReconnect: (closeEvent) => true,
    });

    // 在组件加载时，获取一次历史消息
    useEffect(() => {
        api.getChatHistory(rfqId).then(response => {
            setMessageHistory(response.data);
        }).catch(err => console.error("Failed to fetch chat history:", err));
    }, [rfqId]);

    // **【关键修复】** 当收到任何新消息 (lastMessage) 时，将其添加到历史记录中
    useEffect(() => {
        if (lastMessage !== null) {
            // 后端广播的消息格式为 "UserFullName(CompanyName): message"
            // 我们将其包装成一个对象以便渲染
            setMessageHistory(prev => [...prev, {
                isLive: true, // 标记为实时消息
                data: lastMessage.data,
                id: Date.now() // 使用时间戳作为临时key
            }]);
        }
    }, [lastMessage]);

    // 自动滚动到聊天框底部
    useEffect(() => {
        if (chatBoxRef.current) {
            chatBoxRef.current.scrollTop = chatBoxRef.current.scrollHeight;
        }
    }, [messageHistory]);

    const handleSendMessage = (e) => {
        e.preventDefault();
        if (message.trim()) {
            sendMessage(message);
            setMessage('');
        }
    };

    const connectionStatus = {
        [ReadyState.CONNECTING]: 'Connecting',
        [ReadyState.OPEN]: 'Open',
        [ReadyState.CLOSING]: 'Closing',
        [ReadyState.CLOSED]: 'Closed',
        [ReadyState.UNINSTANTIATED]: 'Uninstantiated',
    }[readyState];

    return (
        <div style={{border: '1px solid #ccc', borderRadius: '5px', marginTop: '2rem'}}>
            <div style={{padding: '1rem', borderBottom: '1px solid #ccc', background: '#f8f9fa'}}>
                <h4>RFQ Chat Room (Status: <span style={{fontWeight: 'normal'}}>{connectionStatus}</span>)</h4>
            </div>
            <div ref={chatBoxRef} style={{height: '300px', overflowY: 'auto', padding: '1rem', background: '#fff'}}>
                {messageHistory.map(msg => (
                    <div key={msg.id} style={{marginBottom: '0.5rem'}}>
                        {/* 如果是历史消息，有完整的用户信息 */}
                        {msg.user_full_name ? (
                            <span><strong>{msg.user_full_name} ({msg.company_name}):</strong> {msg.message_text}</span>
                        ) : (
                            // 如果是实时消息 (简单文本)
                            <span>{msg.data}</span>
                        )}
                    </div>
                ))}
            </div>
            <form onSubmit={handleSendMessage} style={{display: 'flex', padding: '1rem', borderTop: '1px solid #ccc'}}>
                <input
                    type="text"
                    value={message}
                    onChange={e => setMessage(e.target.value)}
                    placeholder="Type a message..."
                    style={{flexGrow: 1, padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px'}}
                    disabled={readyState !== ReadyState.OPEN}
                />
                <button type="submit" className="btn btn-primary" style={{marginLeft: '0.5rem'}} disabled={readyState !== ReadyState.OPEN}>
                    Send
                </button>
            </form>
        </div>
    );
}

export default ChatBox;