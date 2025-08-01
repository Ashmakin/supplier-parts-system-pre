// src/App.jsx

import React from 'react';
import { Routes, Route, Link, useNavigate } from 'react-router-dom';
import { useAuth } from './context/AuthContext';
import { useDisclosure } from '@mantine/hooks';
import { AppShell, Burger, Group, Button, Text, UnstyledButton } from '@mantine/core';

// 导入所有页面组件
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';
import RegisterPage from './pages/RegisterPage';
import DashboardPage from './pages/DashboardPage';
import OrdersPage from './pages/OrdersPage';
import CompanyProfilePage from './pages/CompanyProfilePage';
import MyProfilePage from './pages/MyProfilePage';
import RfqDetailPage from './pages/RfqDetailPage';
import AdminPage from './pages/AdminPage';
import ProtectedRoute from './components/ProtectedRoute';
import AdminRoute from './components/AdminRoute';
import PaymentSuccessPage from './pages/PaymentSuccessPage';
import NotificationBell from "./components/NotificationBell.jsx";


function App() {
    // Mantine hook for controlling the mobile navigation sidebar
    const [opened, { toggle }] = useDisclosure();
    const { user, logoutUser } = useAuth();
    const navigate = useNavigate();

    const handleLogout = () => {
        logoutUser();
        navigate('/');
    };

    const linkStyle = { textDecoration: 'none' };

    // Reusable component for navigation links to avoid repetition
    const NavLinks = ({ isMobile }) => {
        // When a mobile link is clicked, close the sidebar
        const onLinkClick = isMobile ? toggle : () => {};

        return (
            <>
                {user ? (
                    <>
                        <Link to="/dashboard" style={linkStyle} onClick={onLinkClick}><Button fullWidth={isMobile} variant="subtle">Dashboard</Button></Link>
                        <Link to="/orders" style={linkStyle} onClick={onLinkClick}><Button fullWidth={isMobile} variant="subtle">Orders</Button></Link>
                        <Link to="/profile" style={linkStyle} onClick={onLinkClick}><Button fullWidth={isMobile} variant="subtle">My Profile</Button></Link>

                        {user.is_admin && (
                            <Link to="/admin" style={linkStyle} onClick={onLinkClick}>
                                <Button fullWidth={isMobile} variant="light" color="red">Admin Panel</Button>
                            </Link>
                        )}

                        <Button onClick={handleLogout} variant="outline">Logout</Button>
                    </>
                ) : (
                    <>
                        <Link to="/login" style={linkStyle} onClick={onLinkClick}><Button fullWidth={isMobile} variant="default">Login</Button></Link>
                        <Link to="/register" style={linkStyle} onClick={onLinkClick}><Button fullWidth={isMobile} variant="filled">Register</Button></Link>
                    </>
                )}
            </>
        );
    };

    return (
        <AppShell
            header={{ height: 60 }}
            navbar={{
                width: 250,
                breakpoint: 'sm',
                collapsed: { mobile: !opened },
            }}
            padding="md"
        >
            {/* 应用的头部 (Header) */}
            <AppShell.Header>
                <Group h="100%" px="md" justify="space-between">
                    <Group>
                        <Burger opened={opened} onClick={toggle} hiddenFrom="sm" size="sm" />
                        <UnstyledButton component={Link} to="/">
                            <Text size="xl" weight={700}
                                  variant="gradient"
                                  gradient={{ from: 'indigo', to: 'cyan', deg: 45 }}>
                                SCCP
                            </Text>
                        </UnstyledButton>
                    </Group>

                    {/* 桌面端导航链接 */}
                    <Group visibleFrom="sm">
                        {user ? (
                            // 登录后视图
                            <Group>
                                <Button component={Link} to="/dashboard" variant="subtle">Dashboard</Button>
                                <Button component={Link} to="/orders" variant="subtle">Orders</Button>
                                <Button component={Link} to="/profile" variant="subtle">My Profile</Button>
                                {user.is_admin && (
                                    <Button component={Link} to="/admin" variant="light" color="red">Admin Panel</Button>
                                )}
                                <NotificationBell />
                                <Button onClick={handleLogout} variant="outline">Logout</Button>
                            </Group>
                        ) : (
                            // 登录前视图
                            <Group>
                                <Button component={Link} to="/login" variant="default">Login</Button>
                                <Button component={Link} to="/register" variant="filled">Register</Button>
                            </Group>
                        )}
                    </Group>
                </Group>
            </AppShell.Header>

            {/* 移动端视图的侧边栏 (Navbar) */}
            <AppShell.Navbar p="md">
                <Group direction="column" grow>
                    <NavLinks isMobile={true} />
                </Group>
            </AppShell.Navbar>

            {/* 应用的主内容区域，所有页面都在这里渲染 */}
            <AppShell.Main>
                <Routes>
                    <Route path="/" element={<HomePage />} />
                    <Route path="/login" element={<LoginPage />} />
                    <Route path="/register" element={<RegisterPage />} />
                    <Route path="/payment/success" element={ <ProtectedRoute><PaymentSuccessPage /></ProtectedRoute> } />
                    <Route path="/dashboard" element={ <ProtectedRoute><DashboardPage /></ProtectedRoute> } />
                    <Route path="/orders" element={ <ProtectedRoute><OrdersPage /></ProtectedRoute> } />
                    <Route path="/profile" element={ <ProtectedRoute><MyProfilePage /></ProtectedRoute> } />
                    <Route path="/companies/:companyId" element={ <ProtectedRoute><CompanyProfilePage /></ProtectedRoute> } />
                    <Route path="/rfqs/:rfqId" element={ <ProtectedRoute><RfqDetailPage /></ProtectedRoute> } />
                    <Route path="/me" element={ <ProtectedRoute><MyProfilePage /></ProtectedRoute> } />
                    <Route path="/admin" element={ <AdminRoute><AdminPage /></AdminRoute> } />

                </Routes>
            </AppShell.Main>
        </AppShell>
    );
}

export default App;