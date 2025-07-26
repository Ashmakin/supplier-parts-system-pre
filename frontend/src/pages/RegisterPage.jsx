import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import * as api from '../api';

function RegisterPage() {
    const [formData, setFormData] = useState({
        company_name: '',
        company_type: 'BUYER',
        email: '',
        password: '',
        full_name: '',
    });
    const [error, setError] = useState('');
    const navigate = useNavigate();

    const handleChange = (e) => {
        setFormData({ ...formData, [e.target.name]: e.target.value });
    };

    const handleSubmit = async (e) => {
        e.preventDefault();
        setError('');
        if (formData.password.length < 6) {
            setError('Password must be at least 6 characters long.');
            return;
        }
        try {
            await api.register(formData);
            alert('Registration successful! Please log in.');
            navigate('/login');
        } catch (err) {
            setError('Registration failed. The email or company name may already be in use.');
            console.error(err);
        }
    };

    return (
        <div className="form-container">
            <h1>Create Account</h1>
            <form onSubmit={handleSubmit}>
                <div className="form-group">
                    <label>Company Name</label>
                    <input type="text" name="company_name" onChange={handleChange} required />
                </div>
                <div className="form-group">
                    <label>Account Type</label>
                    <select name="company_type" value={formData.company_type} onChange={handleChange}>
                        <option value="BUYER">I'm a Buyer (I need parts)</option>
                        <option value="SUPPLIER">I'm a Supplier (I make parts)</option>
                    </select>
                </div>
                <div className="form-group">
                    <label>Full Name</label>
                    <input type="text" name="full_name" onChange={handleChange} required />
                </div>
                <div className="form-group">
                    <label>Email</label>
                    <input type="email" name="email" onChange={handleChange} required />
                </div>
                <div className="form-group">
                    <label>Password</label>
                    <input type="password" name="password" onChange={handleChange} required />
                </div>
                <button type="submit" className="btn btn-primary btn-block">Register</button>
                {error && <p className="error-message">{error}</p>}
            </form>
        </div>
    );
}

export default RegisterPage;