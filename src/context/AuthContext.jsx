// src/context/AuthContext.jsx

import React, { createContext, useState, useEffect, useContext } from 'react';
import { jwtDecode } from 'jwt-decode';
import * as api from '../api';

const AuthContext = createContext(null);

export const AuthProvider = ({ children }) => {
    const [user, setUser] = useState(null);
    // 【关键修复 #1】添加一个加载状态，初始为 true
    const [isInitializing, setIsInitializing] = useState(true);

    useEffect(() => {
        const token = localStorage.getItem('token');
        try {
            if (token) {
                const decodedUser = jwtDecode(token);
                if (decodedUser.exp * 1000 > Date.now()) {
                    setUser(decodedUser);
                } else {
                    localStorage.removeItem('token');
                }
            }
        } catch (error) {
            console.error("Invalid token found in localStorage, removing it.", error);
            localStorage.removeItem('token');
        } finally {
            // 【关键修复 #2】无论成功还是失败，在检查完成后都将加载状态设为 false
            setIsInitializing(false);
        }
    }, []);

    const loginUser = async (credentials) => {
        const response = await api.login(credentials);
        const { token } = response.data;
        localStorage.setItem('token', token);
        const decodedUser = jwtDecode(token);
        setUser(decodedUser);
        return decodedUser;
    };

    const logoutUser = () => {
        localStorage.removeItem('token');
        setUser(null);
    };

    // 【关键修复 #3】将 isInitializing 状态也提供给所有子组件
    const value = { user, isInitializing, loginUser, logoutUser };

    // 在初始化完成前，不渲染任何子组件，避免竞争条件
    return (
        <AuthContext.Provider value={value}>
            {!isInitializing && children}
        </AuthContext.Provider>
    );
};

export const useAuth = () => {
    return useContext(AuthContext);
};