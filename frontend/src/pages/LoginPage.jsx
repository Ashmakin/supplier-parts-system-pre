// src/pages/LoginPage.jsx

import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
// --- 导入Mantine组件 ---
import { TextInput, PasswordInput, Button, Paper, Title, Container, Alert } from '@mantine/core';
import { IconAlertCircle } from '@tabler/icons-react'; // Mantine使用Tabler图标

function LoginPage() {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const { loginUser } = useAuth();
    const navigate = useNavigate();

    const handleSubmit = async (e) => {
        e.preventDefault();
        setError('');
        setIsLoading(true);
        try {
            await loginUser({ email, password });
            navigate('/dashboard');
        } catch (err) {
            setError('Failed to login. Please check your credentials.');
            console.error(err);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <Container size={420} my={40}>
            <Title align="center">
                Welcome back!
            </Title>

            <Paper withBorder shadow="md" p={30} mt={30} radius="md">
                <form onSubmit={handleSubmit}>
                    <TextInput
                        label="Email"
                        placeholder="your@email.com"
                        required
                        value={email}
                        onChange={(event) => setEmail(event.currentTarget.value)}
                    />
                    <PasswordInput
                        label="Password"
                        placeholder="Your password"
                        required
                        mt="md"
                        value={password}
                        onChange={(event) => setPassword(event.currentTarget.value)}
                    />
                    {error && (
                        <Alert icon={<IconAlertCircle size="1rem" />} title="Login Failed" color="red" mt="md">
                            {error}
                        </Alert>
                    )}
                    <Button fullWidth mt="xl" type="submit" loading={isLoading}>
                        Sign in
                    </Button>
                </form>
            </Paper>
        </Container>
    );
}

export default LoginPage;