import React, { createContext, useState, useEffect, useContext, useCallback } from 'react'; // <-- 导入 useCallback
import useWebSocket, { ReadyState } from 'react-use-websocket';
import { useAuth } from './AuthContext';
import * as api from '../api';

const NotificationContext = createContext(null);

export const NotificationProvider = ({ children }) => {
    const { user } = useAuth();
    const [notifications, setNotifications] = useState([]);
    const [socketUrl, setSocketUrl] = useState(null);

    useEffect(() => {
        if (user) {
            const token = localStorage.getItem('token');
            setSocketUrl(`ws://127.0.0.1:8080/ws?token=${token}`);
        } else {
            setSocketUrl(null);
        }
    }, [user]);

    const { sendMessage, lastMessage, readyState } = useWebSocket(socketUrl, {
        shouldReconnect: (closeEvent) => true,
    });

    useEffect(() => {
        if (user) {
            api.getNotifications()
                .then(response => setNotifications(Array.isArray(response.data) ? response.data : []))
                .catch(err => console.error("Failed to fetch notifications:", err));
        }
    }, [user]);

    useEffect(() => {
        if (lastMessage !== null) {
            const messageData = lastMessage.data;
            if (messageData.startsWith("notification|")) {
                try {
                    const newNotification = JSON.parse(messageData.substring(13));
                    setNotifications(prev => [newNotification, ...prev]);
                } catch (e) {
                    console.error("Failed to parse notification JSON:", e);
                }
            }
        }
    }, [lastMessage]);
    // --- 【关键新增】提供给子组件的状态更新函数 ---
    const markOneAsRead = useCallback(async (notificationId) => {
        // 乐观更新：立即在UI上将通知标记为已读
        setNotifications(prev => prev.map(n =>
            n.id === notificationId ? { ...n, is_read: true } : n
        ));
        // 然后在后台发送API请求
        try {
            await api.markNotificationAsRead(notificationId);
        } catch (error) {
            console.error("Failed to mark notification as read on server:", error);
            // 如果API失败，可以考虑将UI状态回滚
        }
    }, []);

    const markAllAsRead = useCallback(async () => {
        // 乐观更新：立即在UI上将所有通知标记为已读
        setNotifications(prev => prev.map(n => ({ ...n, is_read: true })));
        try {
            await api.markAllNotificationsAsRead();
        } catch (error) {
            console.error("Failed to mark all as read on server:", error);
        }
    }, []);

    // --- 【关键修复】使用 useCallback 包裹所有提供给外部的函数 ---
    const joinRoom = useCallback((rfqId) => {
        if (readyState === ReadyState.OPEN) sendMessage(`JOIN|${rfqId}`);
    }, [readyState, sendMessage]);

    const leaveRoom = useCallback((rfqId) => {
        if (readyState === ReadyState.OPEN) sendMessage(`LEAVE|${rfqId}`);
    }, [readyState, sendMessage]);

    const sendChatMessage = useCallback((rfqId, msg) => {
        if (readyState === ReadyState.OPEN) sendMessage(`CHAT|${rfqId}|${msg}`);
    }, [readyState, sendMessage]);

    const unreadCount = notifications.filter(n => !n.is_read).length;

    const value = {
        notifications,
        unreadCount,
        readyState,
        lastMessage,
        joinRoom,
        leaveRoom,
        sendChatMessage,
        markOneAsRead,
        markAllAsRead,
    };

    return (
        <NotificationContext.Provider value={value}>
            {children}
        </NotificationContext.Provider>
    );
};

export const useNotifications = () => {
    return useContext(NotificationContext);
};