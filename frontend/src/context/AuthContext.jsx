
import React, { createContext, useState, useEffect, useContext } from 'react';
import { jwtDecode } from 'jwt-decode'; // 注意导入名称的变化
import * as api from '../api';

// 创建Auth Context
const AuthContext = createContext(null);

// 创建AuthProvider组件
export const AuthProvider = ({ children }) => {
    const [user, setUser] = useState(null);

    // 在组件加载时，检查localStorage中是否有有效的token
    useEffect(() => {
        const token = localStorage.getItem('token');
        if (token) {
            try {
                const decodedUser = jwtDecode(token);
                // 检查token是否过期
                if (decodedUser.exp * 1000 > Date.now()) {
                    setUser(decodedUser);
                } else {
                    // 如果过期则清除
                    localStorage.removeItem('token');
                }
            } catch (error) {
                console.error("Invalid token found in localStorage", error);
                localStorage.removeItem('token');
            }
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

    return (
        <AuthContext.Provider value={{ user, loginUser, logoutUser }}>
            {children}
        </AuthContext.Provider>
    );
};

// 创建一个自定义hook，方便其他组件使用AuthContext
export const useAuth = () => {
    return useContext(AuthContext);
};