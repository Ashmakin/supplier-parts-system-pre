// src/pages/RegisterPage.jsx

import React, { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import * as api from '../api';
import { TextInput, PasswordInput, Button, Paper, Title, Container, Alert, Select, Text, Anchor } from '@mantine/core';
import { IconAlertCircle } from '@tabler/icons-react';

function RegisterPage() {
    const [formData, setFormData] = useState({
        company_name: '',
        company_type: 'BUYER',
        city: '', // <-- 新增 state 字段
        email: '',
        password: '',
        full_name: '',
    });
    const [error, setError] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const navigate = useNavigate();

    const handleChange = (name, value) => {
        setFormData(prev => ({ ...prev, [name]: value }));
    };

    const handleSubmit = async (e) => {
        e.preventDefault();
        setError('');
        setIsLoading(true);
        if (formData.password.length < 6) {
            setError('Password must be at least 6 characters long.');
            setIsLoading(false);
            return;
        }
        try {
            // formData 现在包含了 city，会被自动发送到后端
            await api.register(formData);
            alert('Registration successful! Please log in.');
            navigate('/login');
        } catch (err) {
            setError('Registration failed. The email or company name may already be in use.');
            console.error(err);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <Container size={420} my={40}>
            <Title align="center">
                Join the Platform
            </Title>
            <Text color="dimmed" size="sm" align="center" mt={5}>
                Already have an account?{' '}
                <Anchor component={Link} to="/login" size="sm">
                    Sign in
                </Anchor>
            </Text>

            <Paper withBorder shadow="md" p={30} mt={30} radius="md">
                <form onSubmit={handleSubmit}>
                    <TextInput
                        label="Company Name"
                        placeholder="Your Company Inc."
                        required
                        value={formData.company_name}
                        onChange={(event) => handleChange('company_name', event.currentTarget.value)}
                    />
                    <Select
                        label="Account Type"
                        placeholder="I am a..."
                        required
                        mt="md"
                        value={formData.company_type}
                        onChange={(value) => handleChange('company_type', value)}
                        data={[
                            { value: 'BUYER', label: "Buyer (I need parts)" },
                            { value: 'SUPPLIER', label: "Supplier (I make parts)" },
                        ]}
                    />
                    {/* --- 【关键修改】新增城市输入框 --- */}
                    <TextInput
                        label="City"
                        placeholder="e.g., Shanghai"
                        required
                        mt="md"
                        value={formData.city}
                        onChange={(event) => handleChange('city', event.currentTarget.value)}
                    />
                    <TextInput
                        label="Full Name"
                        placeholder="John Doe"
                        required
                        mt="md"
                        value={formData.full_name}
                        onChange={(event) => handleChange('full_name', event.currentTarget.value)}
                    />
                    <TextInput
                        label="Email"
                        placeholder="your@email.com"
                        required
                        mt="md"
                        type="email"
                        value={formData.email}
                        onChange={(event) => handleChange('email', event.currentTarget.value)}
                    />
                    <PasswordInput
                        label="Password"
                        placeholder="Your password"
                        required
                        mt="md"
                        value={formData.password}
                        onChange={(event) => handleChange('password', event.currentTarget.value)}
                    />

                    {error && (
                        <Alert icon={<IconAlertCircle size="1rem" />} title="Error" color="red" mt="md">
                            {error}
                        </Alert>
                    )}

                    <Button fullWidth mt="xl" type="submit" loading={isLoading}>
                        Register
                    </Button>
                </form>
            </Paper>
        </Container>
    );
}

export default RegisterPage;