// src/pages/CompanyProfilePage.jsx

import React, { useState, useEffect, useCallback } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import * as api from '../api';

function CompanyProfilePage() {
    const { companyId } = useParams();
    const { user } = useAuth();
    const [profile, setProfile] = useState(null);
    const [isLoading, setIsLoading] = useState(true);
    const [isEditing, setIsEditing] = useState(false);
    const [description, setDescription] = useState('');

    const fetchProfile = useCallback(async () => {
        setIsLoading(true);
        try {
            const response = await api.getCompanyProfile(companyId);
            setProfile(response.data);
            setDescription(response.data.description || '');
        } catch (error) {
            console.error("Failed to fetch profile", error);
        } finally {
            setIsLoading(false);
        }
    }, [companyId]);

    useEffect(() => {
        fetchProfile();
    }, [fetchProfile]);

    const handleUpdateDescription = async (e) => {
        e.preventDefault();
        try {
            await api.updateCompanyProfile(companyId, { description });
            alert("Profile updated!");
            setIsEditing(false);
            fetchProfile(); // 重新获取最新数据
        } catch (error) {
            console.error("Failed to update profile", error);
            alert("Update failed.");
        }
    };

    // 检查当前登录用户是否是该公司页面的所有者
    const isOwner = user && user.company_id === parseInt(companyId);

    if (isLoading) return <div className="container">Loading profile...</div>;
    if (!profile) return <div className="container">Company not found.</div>;

    return (
        <div>
            <Link to="/dashboard">&larr; Back to Dashboard</Link>
            <div style={{ background: 'white', padding: '2rem', marginTop: '1rem', borderRadius: 'var(--border-radius)', boxShadow: '0 2px 4px rgba(0,0,0,0.05)' }}>
                <h1>{profile.name}</h1>
                <p><strong>Type:</strong> {profile.company_type}</p>
                <p><strong>City:</strong> {profile.city || 'Not specified'}</p>
                <hr style={{ margin: '2rem 0' }} />
                <h3>About Us</h3>

                {isEditing ? (
                    <form onSubmit={handleUpdateDescription}>
                        <textarea
                            value={description}
                            onChange={(e) => setDescription(e.target.value)}
                            rows="10"
                            style={{width: '100%', padding: '0.5rem'}}
                        ></textarea>
                        <div style={{marginTop: '1rem'}}>
                            <button type="submit" className="btn btn-primary">Save Changes</button>
                            <button type="button" onClick={() => setIsEditing(false)} className="btn" style={{marginLeft: '1rem'}}>Cancel</button>
                        </div>
                    </form>
                ) : (
                    <div>
                        <p style={{whiteSpace: 'pre-wrap'}}>{profile.description || 'No description provided.'}</p>
                        {isOwner && (
                            <button onClick={() => setIsEditing(true)} className="btn">Edit Description</button>
                        )}
                    </div>
                )}
            </div>
        </div>
    );
}

export default CompanyProfilePage;