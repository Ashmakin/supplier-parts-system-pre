import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App.jsx';
import './index.css';
import { BrowserRouter } from 'react-router-dom';
import { AuthProvider } from './context/AuthContext.jsx';
// --- 新增导入 ---
import { MantineProvider } from '@mantine/core';
import '@mantine/core/styles.css'; // 导入Mantine的核心样式


ReactDOM.createRoot(document.getElementById('root')).render(
    <React.StrictMode>
        <BrowserRouter>
            {/* 用 MantineProvider 包裹 AuthProvider */}
            <MantineProvider withGlobalStyles withNormalizeCSS>
                <AuthProvider>
                    <App />
                </AuthProvider>
            </MantineProvider>
        </BrowserRouter>
    </React.StrictMode>
);