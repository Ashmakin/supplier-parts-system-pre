// src/components/ChatBox.jsx

import React, { useState, useEffect, useRef } from 'react';
import { useNotifications } from '../context/NotificationContext';
import * as api from '../api';
import { Paper, Text, TextInput, Button, Group, ScrollArea, Title, Alert, Center, Loader } from '@mantine/core';
import { ReadyState } from 'react-use-websocket';

function ChatBox({ rfqId }) {
    const [messageHistory, setMessageHistory] = useState([]);
    const [message, setMessage] = useState('');
    const [isLoadingHistory, setIsLoadingHistory] = useState(true);
    const scrollAreaRef = useRef(null);

    // 从全局Context获取WebSocket相关的所有功能
    const { readyState, lastMessage, joinRoom, leaveRoom, sendChatMessage } = useNotifications();

    // 1. 在组件加载时，获取历史聊天记录
    useEffect(() => {
        if (rfqId) {
            setIsLoadingHistory(true);
            api.getChatHistory(rfqId)
                .then(response => {
                    // 确保我们总是处理一个数组
                    setMessageHistory(Array.isArray(response.data) ? response.data : []);
                })
                .catch(err => {
                    console.error("Failed to fetch chat history:", err);
                    setMessageHistory([]); // 出错时设置为空数组
                })
                .finally(() => setIsLoadingHistory(false));
        }
    }, [rfqId]);

    // 2. 当WebSocket连接成功后，加入房间；当组件卸载时，离开房间
    useEffect(() => {
        if (rfqId && readyState === ReadyState.OPEN) {
            joinRoom(rfqId);
        }
        // 返回一个清理函数，在组件卸载（离开页面）时执行
        return () => {
            if (rfqId && readyState === ReadyState.OPEN) {
                leaveRoom(rfqId);
            }
        };
    }, [rfqId, readyState, joinRoom, leaveRoom]);

    // 3. 【关键修复】监听来自服务器的实时消息
    useEffect(() => {
        if (lastMessage !== null) {
            const messageData = lastMessage.data;
            // 确保只处理聊天消息
            if (messageData.startsWith("chat|")) {
                const content = messageData.substring(5); // 移除 "chat|" 前缀

                const newLiveMessage = {
                    id: Date.now(), // 使用时间戳作为临时key
                    isLive: true,
                    // 后端广播的消息格式为 "User(Company): message"
                    message_text: content,
                };
                // 更新state以在UI上显示新消息
                setMessageHistory(prev => [...prev, newLiveMessage]);
            }
        }
    }, [lastMessage]);

    // 4. 自动滚动到聊天框底部
    useEffect(() => {
        if (scrollAreaRef.current) {
            scrollAreaRef.current.scrollTo({ y: scrollAreaRef.current.scrollHeight });
        }
    }, [messageHistory]);


    const handleSendMessage = (e) => {
        e.preventDefault();
        if (message.trim()) {
            // 使用Context提供的函数发送消息
            sendChatMessage(rfqId, message);
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
        <Paper withBorder p="md" radius="md" mt="xl">
            <Group position="apart" mb="md">
                <Title order={4}>RFQ Chat Room</Title>
                <Text size="xs" color="dimmed">Status: {connectionStatus}</Text>
            </Group>
            <ScrollArea h={300} style={{ border: '1px solid #eee', borderRadius: '4px' }} viewportRef={scrollAreaRef}>
                <div style={{padding: '0.5rem'}}>
                    {isLoadingHistory ? <Center><Loader size="sm" /></Center> :
                        messageHistory.length === 0 ? <Text c="dimmed" ta="center" p="md">No messages yet. Start the conversation!</Text> :
                            messageHistory.map(msg => (
                                <div key={msg.id} style={{marginBottom: '0.5rem'}}>
                                    <Text size="sm">
                                        {msg.isLive ?
                                            // 实时消息是已经格式化好的字符串
                                            msg.message_text
                                            :
                                            // 历史消息是包含用户信息的对象
                                            <><strong>{msg.user_full_name} ({msg.company_name}):</strong> {msg.message_text}</>
                                        }
                                    </Text>
                                </div>
                            ))}
                </div>
            </ScrollArea>
            <form onSubmit={handleSendMessage}>
                <Group mt="md">
                    <TextInput
                        style={{ flex: 1 }}
                        placeholder="Type a message..."
                        value={message}
                        onChange={e => setMessage(e.target.value)}
                        disabled={readyState !== ReadyState.OPEN}
                    />
                    <Button type="submit" disabled={readyState !== ReadyState.OPEN}>Send</Button>
                </Group>
            </form>
        </Paper>
    );
}

export default ChatBox;