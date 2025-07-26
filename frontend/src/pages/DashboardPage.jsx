import React, { useState, useEffect, useCallback } from 'react';
import { useAuth } from '../context/AuthContext';
import * as api from '../api';
import { Link } from 'react-router-dom';
import BuyerAnalytics from '../components/BuyerAnalytics'; // <-- 新增导入

/**
 * 创建RFQ的表单组件
 */
function CreateRfqForm({ onRfqCreated }) {
    const [title, setTitle] = useState('');
    const [description, setDescription] = useState('');
    const [quantity, setQuantity] = useState(1);
    const [attachment, setAttachment] = useState(null);
    const [isSubmitting, setIsSubmitting] = useState(false);

    const handleFileChange = (e) => {
        setAttachment(e.target.files[0]);
    };

    const handleSubmit = async (e) => {
        e.preventDefault();
        setIsSubmitting(true);

        const formData = new FormData();
        formData.append('title', title);
        formData.append('description', description);
        formData.append('quantity', Number(quantity));
        if (attachment) {
            formData.append('attachment', attachment);
        }

        try {
            await api.createRfq(formData);
            alert('RFQ created successfully!');
            // 重置表单
            setTitle('');
            setDescription('');
            setQuantity(1);
            setAttachment(null);
            e.target.reset(); // 重置文件输入框
            onRfqCreated(); // 通知父组件刷新列表
        } catch (error) {
            console.error("Failed to create RFQ", error);
            alert('Failed to create RFQ.');
        } finally {
            setIsSubmitting(false);
        }
    };

    return (
        <div className="form-container" style={{maxWidth: '100%', margin: '2rem 0', boxShadow: 'none', padding: 0}}>
            <h3>Create a new Request for Quotation</h3>
            <form onSubmit={handleSubmit}>
                <div className="form-group">
                    <label>Title</label>
                    <input type="text" value={title} onChange={e => setTitle(e.target.value)} required disabled={isSubmitting} />
                </div>
                <div className="form-group">
                    <label>Description / Specs</label>
                    <textarea value={description} onChange={e => setDescription(e.target.value)} rows="3" disabled={isSubmitting}></textarea>
                </div>
                <div className="form-group">
                    <label>Quantity</label>
                    <input type="number" value={quantity} onChange={e => setQuantity(e.target.value)} required min="1" disabled={isSubmitting} />
                </div>
                <div className="form-group">
                    <label>Attachment (PDF, CAD, etc.)</label>
                    <input type="file" onChange={handleFileChange} disabled={isSubmitting} />
                </div>
                <button type="submit" className="btn btn-primary" disabled={isSubmitting}>
                    {isSubmitting ? 'Submitting...' : 'Submit RFQ'}
                </button>
            </form>
        </div>
    );
}

/**
 * 显示RFQ列表的组件
 */
function FilterForm({ onSearch }) {
    const [searchTerm, setSearchTerm] = useState('');
    const [city, setCity] = useState('');

    const handleSearch = (e) => {
        e.preventDefault();
        onSearch({ search: searchTerm, city });
    };

    return (
        <form onSubmit={handleSearch} style={{ display: 'flex', gap: '1rem', alignItems: 'center', margin: '2rem 0', background: '#fff', padding: '1.5rem', borderRadius: 'var(--border-radius)', boxShadow: '0 2px 4px rgba(0,0,0,0.05)' }}>
            <input
                type="text"
                placeholder="Search by keyword..."
                value={searchTerm}
                onChange={e => setSearchTerm(e.target.value)}
                style={{ flexGrow: 2, padding: '0.75rem' }}
            />
            <input
                type="text"
                placeholder="Filter by city..."
                value={city}
                onChange={e => setCity(e.target.value)}
                style={{ flexGrow: 1, padding: '0.75rem' }}
            />
            <button type="submit" className="btn btn-primary">Search</button>
        </form>
    );
}


/**
 * 主仪表盘页面组件
 */
function DashboardPage() {
    const { user } = useAuth();
    const [rfqs, setRfqs] = useState([]);
    const [isLoading, setIsLoading] = useState(true);
    // 新增 state 来保存筛选条件
    const [filters, setFilters] = useState({ search: '', city: '' });

    // useCallback 的依赖项中加入 filters
    const fetchRfqs = useCallback(async () => {
        if (!user) return;
        setIsLoading(true);
        try {
            // 将筛选条件传递给API调用
            const response = await api.getRfqs(filters);

            // 前端的过滤逻辑现在可以移除了，因为后端已经处理了
            setRfqs(response.data);

        } catch (error) {
            console.error("Failed to fetch RFQs", error);
            setRfqs([]);
        } finally {
            setIsLoading(false);
        }
    }, [user, filters]); // <-- filters 作为依赖

    useEffect(() => {
        fetchRfqs();
    }, [fetchRfqs]);

    // 处理搜索的函数
    const handleSearch = (newFilters) => {
        setFilters(newFilters);
    };

    if (!user) return <div className="container">Loading user data...</div>;

    return (
        <div>
            <h1>Dashboard</h1>
            <p className="lead">Welcome back, <strong>{user.company_type}</strong> from company ID #{user.company_id}.</p>
            <hr style={{margin: '2rem 0'}} />
            {/* --- 新增：只为采购方显示分析模块 --- */}
            {user.company_type === 'BUYER' && (
                <div style={{marginBottom: '2rem'}}>
                    <BuyerAnalytics />
                </div>
            )}
            {user.company_type === 'BUYER' && <CreateRfqForm onRfqCreated={fetchRfqs} />}

            <div style={{marginTop: '2rem'}}>
                <h3>{user.company_type === 'BUYER' ? "My Open RFQs" : "Find Open RFQs"}</h3>

                {/* 供应商才能看到搜索栏 */}
                {user.company_type === 'SUPPLIER' && <FilterForm onSearch={handleSearch} />}

                {isLoading ? <p>Loading RFQs...</p> : <RfqList rfqs={rfqs} />}
            </div>
        </div>
    );
}

// 确保 RfqList 组件的代码也更新以显示城市
function RfqList({ rfqs }) {
    if (!rfqs || rfqs.length === 0) {
        return <p>No open RFQs found with the current filters.</p>;
    }
    return (
        <table style={{width: '100%', borderCollapse: 'collapse', fontSize: '0.9rem'}}>
            <thead>
            <tr>
                <th style={{textAlign: 'left', padding: '12px', borderBottom: '2px solid #dee2e6'}}>Title</th>
                <th style={{textAlign: 'left', padding: '12px', borderBottom: '2px solid #dee2e6'}}>Buyer</th>
                <th style={{textAlign: 'left', padding: '12px', borderBottom: '2px solid #dee2e6'}}>City</th> {/* <-- 新增 City 列 */}
                <th style={{textAlign: 'left', padding: '12px', borderBottom: '2px solid #dee2e6'}}>Quantity</th>
                <th style={{textAlign: 'left', padding: '12px', borderBottom: '2px solid #dee2e6'}}>Status</th>
                <th style={{textAlign: 'left', padding: '12px', borderBottom: '2px solid #dee2e6'}}></th>
            </tr>
            </thead>
            <tbody>
            {rfqs.map(rfq => (
                <tr key={rfq.id} style={{borderBottom: '1px solid #dee2e6'}}>
                    <td style={{padding: '12px'}}>{rfq.title}</td>
                    <td style={{padding: '12px'}}>{rfq.buyer_company_name}</td>
                    <td style={{padding: '12px'}}>{rfq.city || 'N/A'}</td> {/* <-- 显示 City 数据 */}
                    <td style={{padding: '12px'}}>{rfq.quantity}</td>
                    <td style={{padding: '12px'}}>{rfq.status}</td>
                    <td style={{padding: '12px', textAlign: 'right'}}>
                        <Link to={`/rfqs/${rfq.id}`} className="btn btn-primary">View Details</Link>
                    </td>
                </tr>
            ))}
            </tbody>
        </table>
    );
}

export default DashboardPage;
