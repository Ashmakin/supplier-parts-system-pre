import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import * as api from '../api';

// 导入所有需要的Mantine组件
import {
    Title,
    Text,
    Paper,
    Button,
    Container,
    PasswordInput,
    Alert,
    Divider,
    Loader,
    Center,
    Group
} from '@mantine/core';
import { IconAlertCircle, IconCircleCheck } from '@tabler/icons-react';

function MyProfilePage() {
    // State for fetching user profile data
    const [profile, setProfile] = useState(null);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState('');

    // State for the password change form
    const [currentPassword, setCurrentPassword] = useState('');
    const [newPassword, setNewPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [isUpdating, setIsUpdating] = useState(false);
    const [passwordMessage, setPasswordMessage] = useState({ type: '', text: '' });


    // Fetch user profile when the component mounts
    useEffect(() => {
        const fetchProfile = async () => {
            setIsLoading(true);
            try {
                const response = await api.getMyProfile();
                setProfile(response.data);
            } catch (err) {
                setError('Failed to load your profile. Please try logging in again.');
                console.error(err);
            } finally {
                setIsLoading(false);
            }
        };
        fetchProfile();
    }, []);

    // Handle the password change form submission
    const handlePasswordChange = async (e) => {
        e.preventDefault();
        setPasswordMessage({ type: '', text: '' }); // Reset message

        // Client-side validation
        if (newPassword !== confirmPassword) {
            setPasswordMessage({ type: 'error', text: 'New passwords do not match.' });
            return;
        }
        if (newPassword.length < 6) {
            setPasswordMessage({ type: 'error', text: 'New password must be at least 6 characters long.' });
            return;
        }

        setIsUpdating(true);
        try {
            await api.changePassword({
                current_password: currentPassword,
                new_password: newPassword,
            });
            setPasswordMessage({ type: 'success', text: 'Password updated successfully! You may need to log in again with your new password.' });
            // Clear form fields on success
            setCurrentPassword('');
            setNewPassword('');
            setConfirmPassword('');
        } catch (err) {
            const errorText = err.response?.data || "An unexpected error occurred.";
            setPasswordMessage({ type: 'error', text: `Error: ${errorText}` });
            console.error(err);
        } finally {
            setIsUpdating(false);
        }
    };

    if (isLoading) {
        return <Center style={{ height: '80vh' }}><Loader /></Center>;
    }

    if (error) {
        return <Alert icon={<IconAlertCircle size="1rem" />} title="Error" color="red">{error}</Alert>;
    }

    return (
        <Container size="md">
            <Title order={1} mb="lg">My Profile</Title>

            {profile && (
                <Paper withBorder shadow="sm" p="xl" radius="md">
                    <Title order={3}>Account Information</Title>
                    <Divider my="md" />
                    <Group mt="sm">
                        <Text fw={500} w={120}>Full Name:</Text>
                        <Text>{profile.full_name}</Text>
                    </Group>
                    <Group mt="sm">
                        <Text fw={500} w={120}>Email:</Text>
                        <Text>{profile.email}</Text>
                    </Group>
                    <Group mt="sm">
                        <Text fw={500} w={120}>Company:</Text>
                        <Button component={Link} to={`/companies/${profile.company_id}`} variant="subtle" compact>
                            {profile.company_name}
                        </Button>
                    </Group>
                </Paper>
            )}

            <Paper withBorder shadow="sm" p="xl" radius="md" mt="xl">
                <Title order={3}>Change Password</Title>
                <Divider my="md" />
                <form onSubmit={handlePasswordChange}>
                    <PasswordInput
                        label="Current Password"
                        placeholder="Enter your current password"
                        value={currentPassword}
                        onChange={e => setCurrentPassword(e.target.value)}
                        required
                    />
                    <PasswordInput
                        label="New Password"
                        placeholder="Enter a new password"
                        mt="md"
                        value={newPassword}
                        onChange={e => setNewPassword(e.target.value)}
                        required
                    />
                    <PasswordInput
                        label="Confirm New Password"
                        placeholder="Confirm your new password"
                        mt="md"
                        value={confirmPassword}
                        onChange={e => setConfirmPassword(e.target.value)}
                        required
                    />

                    {/* Display success or error messages for the password form */}
                    {passwordMessage.text && (
                        <Alert
                            icon={passwordMessage.type === 'success' ? <IconCircleCheck size="1rem" /> : <IconAlertCircle size="1rem" />}
                            title={passwordMessage.type === 'success' ? "Success" : "Error"}
                            color={passwordMessage.type === 'success' ? 'teal' : 'red'}
                            mt="lg"
                        >
                            {passwordMessage.text}
                        </Alert>
                    )}

                    <Button type="submit" mt="xl" loading={isUpdating}>
                        Update Password
                    </Button>
                </form>
            </Paper>
        </Container>
    );
}

export default MyProfilePage;