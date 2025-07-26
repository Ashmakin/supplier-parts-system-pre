import React, { useState, useEffect, useCallback } from 'react';
import { useParams, useNavigate, Link } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import * as api from '../api';
import ChatBox from '../components/ChatBox';
/**
 * 供应商提交报价的表单
 */
function CreateQuoteForm({ rfqId, onQuoteSubmitted }) {
    const [price, setPrice] = useState('');
    const [lead_time_days, setLeadTime] = useState('');
    const [notes, setNotes] = useState('');
    const [isSubmitting, setIsSubmitting] = useState(false);

    const handleSubmit = async (e) => {
        e.preventDefault();
        setIsSubmitting(true);
        try {
            await api.createQuote(rfqId, { price: parseFloat(price), lead_time_days: parseInt(lead_time_days), notes });
            alert('Quote submitted successfully!');
            onQuoteSubmitted(); // 通知父组件刷新
        } catch (error) {
            console.error('Failed to submit quote', error);
            alert('Failed to submit quote.');
        } finally {
            setIsSubmitting(false);
        }
    };

    return (
        <div className="form-container" style={{maxWidth: '100%', margin: '2rem 0', boxShadow: 'none', padding: '2rem', border: '1px solid #ddd', borderRadius: 'var(--border-radius)'}}>
            <h3>Submit Your Quote</h3>
            <form onSubmit={handleSubmit}>
                <div className="form-group">
                    <label>Your Price (per unit)</label>
                    <input type="number" step="0.01" value={price} onChange={e => setPrice(e.target.value)} required disabled={isSubmitting}/>
                </div>
                <div className="form-group">
                    <label>Your Lead Time (in days)</label>
                    <input type="number" value={lead_time_days} onChange={e => setLeadTime(e.target.value)} required disabled={isSubmitting}/>
                </div>
                <div className="form-group">
                    <label>Notes (optional)</label>
                    <textarea value={notes} onChange={e => setNotes(e.target.value)} rows="3" disabled={isSubmitting}></textarea>
                </div>
                <button type="submit" className="btn btn-primary" disabled={isSubmitting}>
                    {isSubmitting ? 'Submitting...' : 'Submit Quote'}
                </button>
            </form>
        </div>
    );
}

/**
 * 采购方看到的报价列表
 */
function QuoteList({ quotes, onAccept }) {
    if (!quotes.length) return <p>No quotes have been received for this RFQ yet.</p>;

    return (
        <div>
            <h3>Received Quotes</h3>
            <div style={{display: 'grid', gap: '1rem'}}>
                {quotes.map(quote => (
                    <div key={quote.id} style={{border: '1px solid #ccc', padding: '1rem', borderRadius: 'var(--border-radius)', background: 'white'}}>
                        <h4>From: {quote.supplier_company_name}</h4>
                        <p><strong>Price:</strong> ${quote.price}</p>
                        <p><strong>Lead Time:</strong> {quote.lead_time_days} days</p>
                        <p><strong>Notes:</strong> {quote.notes || 'N/A'}</p>
                        <p><strong>Status:</strong> {quote.status}</p>
                        {quote.status === 'SUBMITTED' && (
                            <button onClick={() => onAccept(quote.id)} className="btn btn-primary">Accept Quote</button>
                        )}
                    </div>
                ))}
            </div>
        </div>
    );
}

/**
 * 主详情页组件 - 使用了更健壮的数据获取逻辑
 */
function RfqDetailPage() {
    const { rfqId } = useParams();
    const { user } = useAuth();
    const navigate = useNavigate();
    const [rfq, setRfq] = useState(null);
    const [attachments, setAttachments] = useState([]);
    const [quotes, setQuotes] = useState([]);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState('');

    const fetchData = useCallback(async () => {
        if (!user || !rfqId) return;

        setIsLoading(true);
        setError('');

        try {
            // 第一步：只获取最核心的RFQ详情
            const rfqResponse = await api.getRfqById(rfqId);
            const fetchedRfq = rfqResponse.data;
            setRfq(fetchedRfq); // **立即设置RFQ数据**

            // 第二步：分别获取次要数据（附件和报价），并独立处理它们的错误
            try {
                const attachmentsResponse = await api.getRfqAttachments(rfqId);
                setAttachments(attachmentsResponse.data);
            } catch (attachError) {
                console.error("Could not fetch attachments:", attachError);
                // 附件加载失败不应阻塞整个页面
            }

            if (user.company_type === 'BUYER' && user.company_id === fetchedRfq.buyer_company_id) {
                try {
                    const quotesResponse = await api.getQuotesForRfq(rfqId);
                    setQuotes(quotesResponse.data);
                } catch (quoteError) {
                    console.error("Could not fetch quotes:", quoteError);
                    // 报价加载失败也不应阻塞整个页面
                }
            }

        } catch (err) {
            console.error("Failed to fetch main RFQ details", err);
            setError('Failed to load RFQ data. It may not exist or you may not have permission to view it.');
            setRfq(null); // 确保在出错时 rfq 为 null
        } finally {
            setIsLoading(false);
        }
    }, [rfqId, user]);

    useEffect(() => {
        fetchData();
    }, [fetchData]);

    const handleAcceptQuote = async (quoteId) => {
        if (window.confirm("Are you sure you want to accept this quote? This will create a purchase order and this RFQ will be closed to new quotes.")) {
            try {
                await api.acceptQuote(quoteId);
                alert("Quote accepted! A purchase order has been created.");
                navigate('/dashboard');
            } catch (error) {
                console.error("Failed to accept quote", error);
                alert("Failed to accept quote. The RFQ might already be closed.");
            }
        }
    };

    if (isLoading) return <div className="container">Loading details...</div>;
    if (error) return <div className="container error-message">{error}</div>;
    if (!rfq) return <div className="container"><h2>RFQ not found.</h2></div>;

    const isOwner = user.company_type === 'BUYER' && user.company_id === rfq.buyer_company_id;
    const canSupplierQuote = user.company_type === 'SUPPLIER' && rfq.status === 'OPEN';

    return (
        <div>
            <Link to="/dashboard">&larr; Back to Dashboard</Link>
            <h1 style={{marginTop: '1rem'}}>RFQ Detail: {rfq.title}</h1>
            <div style={{background: 'white', padding: '2rem', borderRadius: 'var(--border-radius)', boxShadow: '0 2px 4px rgba(0,0,0,0.05)'}}>
                <p><strong>Status:</strong> {rfq.status}</p>
                <p><strong>Buyer:</strong>
                    <Link to={`/companies/${rfq.buyer_company_id}`} style={{marginLeft: '0.5rem'}}>
                    {rfq.buyer_company_name}</Link>
                </p>
                <p><strong>Quantity Required:</strong> {rfq.quantity}</p>
                <p><strong>Description:</strong> {rfq.description || "No description provided."}</p>

                <h4>Attachments:</h4>
                {attachments.length > 0 ? (
                    <ul>
                        {attachments.map(att => (
                            <li key={att.id}>
                                <a
                                    href={`http://127.0.0.1:8080${att.stored_path.replace('./', '/')}`}
                                    target="_blank"
                                    rel="noopener noreferrer"
                                >
                                    {att.original_filename}
                                </a>
                            </li>
                        ))}
                    </ul>
                ) : (
                    <p>No attachments.</p>
                )}

            </div>

            <hr style={{margin: '2rem 0'}}/>
            {/* --- 新增聊天框组件 --- */}
            <ChatBox rfqId={rfqId} />
            {isOwner && rfq.status === 'AWARDED' && (
                <div style={{padding: '1rem', background: '#e2f0d9', border: '1px solid var(--success-color)', borderRadius: 'var(--border-radius)'}}>
                    This RFQ has been awarded. A purchase order has been created.
                </div>
            )}
            {isOwner && rfq.status === 'OPEN' && <QuoteList quotes={quotes} onAccept={handleAcceptQuote} />}
            {canSupplierQuote && <CreateQuoteForm rfqId={rfq.id} onQuoteSubmitted={fetchData} />}
        </div>
    );
}

export default RfqDetailPage;