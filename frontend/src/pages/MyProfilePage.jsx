// src/pages/MyProfilePage.jsx

import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import * as api from '../api';

function MyProfilePage() {
    const [profile, setProfile] = useState(null);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState('');

    // 密码修改表单的状态
    const [currentPassword, setCurrentPassword] = useState('');
    const [newPassword, setNewPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [passwordMessage, setPasswordMessage] = useState({ type: '', text: '' });


    useEffect(() => {
        const fetchProfile = async () => {
            try {
                const response = await api.getMyProfile();
                setProfile(response.data);
            } catch (err) {
                setError('Failed to load profile.');
                console.error(err);
            } finally {
                setIsLoading(false);
            }
        };
        fetchProfile();
    }, []);

    const handlePasswordChange = async (e) => {
        e.preventDefault();
        setPasswordMessage({ type: '', text: '' }); // 重置消息

        if (newPassword !== confirmPassword) {
            setPasswordMessage({ type: 'error', text: 'New passwords do not match.' });
            return;
        }

        try {
            await api.changePassword({
                current_password: currentPassword,
                new_password: newPassword,
            });
            setPasswordMessage({ type: 'success', text: 'Password updated successfully!' });
            // 清空表单
            setCurrentPassword('');
            setNewPassword('');
            setConfirmPassword('');
        } catch (err) {
            const errorText = err.response?.data || "An unexpected error occurred.";
            setPasswordMessage({ type: 'error', text: `Error: ${errorText}` });
            console.error(err);
        }
    };

    if (isLoading) return <div className="container">Loading profile...</div>;
    if (error) return <div className="container error-message">{error}</div>;

    return (
        <div>
            <h1>My Profile</h1>
            {profile && (
                <div style={{ background: 'white', padding: '2rem', borderRadius: 'var(--border-radius)' }}>
                    <p><strong>Full Name:</strong> {profile.full_name}</p>
                    <p><strong>Email:</strong> {profile.email}</p>
                    <p>
                        <strong>Company:</strong>
                        <Link to={`/companies/${profile.company_id}`} style={{ marginLeft: '0.5rem' }}>
                            {profile.company_name}
                        </Link>
                    </p>
                </div>
            )}

            <div style={{ background: 'white', padding: '2rem', borderRadius: 'var(--border-radius)', marginTop: '2rem' }}>
                <h3>Change Password</h3>
                <form onSubmit={handlePasswordChange}>
                    <div className="form-group">
                        <label>Current Password</label>
                        <input type="password" value={currentPassword} onChange={e => setCurrentPassword(e.target.value)} required />
                    </div>
                    <div className="form-group">
                        <label>New Password</label>
                        <input type="password" value={newPassword} onChange={e => setNewPassword(e.target.value)} required />
                    </div>
                    <div className="form-group">
                        <label>Confirm New Password</label>
                        <input type="password" value={confirmPassword} onChange={e => setConfirmPassword(e.target.value)} required />
                    </div>
                    <button type="submit" className="btn btn-primary">Update Password</button>
                    {passwordMessage.text && (
                        <p style={{ color: passwordMessage.type === 'error' ? 'var(--danger-color)' : 'var(--success-color)', marginTop: '1rem' }}>
                            {passwordMessage.text}
                        </p>
                    )}
                </form>
            </div>
        </div>
    );
}

export default MyProfilePage;