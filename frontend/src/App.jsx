// src/App.jsx
import React from 'react';
import { Routes, Route } from 'react-router-dom';
import Navbar from './components/Navbar';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';
import RegisterPage from './pages/RegisterPage';
import DashboardPage from './pages/DashboardPage';
import ProtectedRoute from './components/ProtectedRoute';
import RfqDetailPage from './pages/RfqDetailPage'; // <-- 新增导入
import OrdersPage from './pages/OrdersPage'; // <-- 新增导入
import CompanyProfilePage from './pages/CompanyProfilePage'; // <-- 新增导入
import MyProfilePage from './pages/MyProfilePage'; // <-- 新增导入

function App() {
    return (
        <>
            <Navbar />
            <div className="container">
                <Routes>
                    <Route path="/" element={<HomePage />} />
                    <Route path="/login" element={<LoginPage />} />
                    <Route path="/register" element={<RegisterPage />} />
                    <Route path="/dashboard" element={ <ProtectedRoute><DashboardPage /></ProtectedRoute> } />
                    {/* --- 新增下面这行 --- */}
                    <Route path="/rfqs/:rfqId" element={ <ProtectedRoute><RfqDetailPage /></ProtectedRoute> } />
                    <Route path="/orders" element={ <ProtectedRoute><OrdersPage /></ProtectedRoute> } /> {/* <-- 新增 */}
                    <Route path="/companies/:companyId" element={ <ProtectedRoute><CompanyProfilePage /></ProtectedRoute> } /> {/* <-- 新增 */}
                    <Route path="/profile" element={ <ProtectedRoute><MyProfilePage /></ProtectedRoute> } /> {/* <-- 新增 */}
                </Routes>
            </div>
        </>
    );
}

export default App;