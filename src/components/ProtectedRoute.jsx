// src/components/ProtectedRoute.jsx

import React from 'react';
import { Navigate } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import { Loader, Center } from '@mantine/core'; // 导入一个加载指示器

function ProtectedRoute({ children }) {
    const { user, isInitializing } = useAuth();

    // 【关键修复 #1】如果正在进行初始化检查，则显示一个加载动画
    if (isInitializing) {
        return (
            <Center style={{ height: '80vh' }}>
                <Loader />
            </Center>
        );
    }

    // 【关键修复 #2】只有在初始化完成后，才检查用户是否存在
    if (!user) {
        // 如果用户不存在，重定向到登录页
        return <Navigate to="/login" replace />;
    }

    // 如果初始化完成且用户存在，则渲染子组件（即受保护的页面）
    return children;
}

export default ProtectedRoute;