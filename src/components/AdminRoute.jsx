// src/components/AdminRoute.jsx
import React from 'react';
import { Navigate } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import { Loader, Center, Text } from '@mantine/core';

function AdminRoute({ children }) {
    const { user, isInitializing } = useAuth();

    if (isInitializing) {
        return <Center style={{ height: '80vh' }}><Loader /></Center>;
    }

    // 检查用户是否存在且是管理员
    if (!user || !user.is_admin) {
        // 可以重定向到首页，或显示一个无权限页面
        return <Navigate to="/" replace />;
    }

    return children;
}

export default AdminRoute;