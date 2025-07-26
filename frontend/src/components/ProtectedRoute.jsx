import React from 'react';
import { Navigate } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';

function ProtectedRoute({ children }) {
    const { user } = useAuth();

    if (!user) {
        // 如果用户未登录，重定向到登录页面
        return <Navigate to="/login" replace />;
    }

    return children;
}

export default ProtectedRoute;