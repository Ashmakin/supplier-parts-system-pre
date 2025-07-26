// src/pages/OrdersPage.jsx

import React, { useState, useEffect, useCallback } from 'react';
import { useAuth } from '../context/AuthContext';
import * as api from '../api';

function OrdersPage() {
    const { user } = useAuth();
    const [orders, setOrders] = useState([]);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState('');

    const fetchOrders = useCallback(async () => {
        setIsLoading(true);
        setError('');
        try {
            const response = await api.getOrders();
            setOrders(response.data);
        } catch (err) {
            console.error("Failed to fetch orders", err);
            setError("Could not load orders.");
        } finally {
            setIsLoading(false);
        }
    }, []);

    useEffect(() => {
        fetchOrders();
    }, [fetchOrders]);

    const handleStatusChange = async (orderId, newStatus) => {
        try {
            await api.updateOrderStatus(orderId, newStatus);
            alert('Order status updated!');
            // 重新获取订单列表以显示最新状态
            fetchOrders();
        } catch (err) {
            console.error('Failed to update status', err);
            alert('Failed to update status.');
        }
    };

    if (isLoading) return <div>Loading orders...</div>;
    if (error) return <div className="error-message">{error}</div>;

    return (
        <div>
            <h1>My Orders</h1>
            <table style={{width: '100%', borderCollapse: 'collapse'}}>
                <thead>
                <tr>
                    <th>Order ID</th>
                    <th>RFQ Title</th>
                    <th>{user.company_type === 'BUYER' ? 'Supplier' : 'Buyer'}</th>
                    <th>Amount</th>
                    <th>Status</th>
                    {user.company_type === 'SUPPLIER' && <th>Update Status</th>}
                </tr>
                </thead>
                <tbody>
                {orders.length === 0 ? (
                    <tr>
                        <td colSpan="6" style={{textAlign: 'center', padding: '1rem'}}>No orders found.</td>
                    </tr>
                ) : (
                    orders.map(order => (
                        <tr key={order.id}>
                            <td>#{order.id}</td>
                            <td>{order.rfq_title}</td>
                            <td>{user.company_type === 'BUYER' ? order.supplier_name : order.buyer_name}</td>
                            <td>${order.total_amount}</td>
                            <td>{order.status}</td>
                            {user.company_type === 'SUPPLIER' && (
                                <td>
                                    <select
                                        defaultValue={order.status}
                                        onChange={(e) => handleStatusChange(order.id, e.target.value)}
                                        disabled={order.status === 'COMPLETED'}
                                    >
                                        <option value="PENDING_CONFIRMATION" disabled>Pending Confirmation</option>
                                        <option value="IN_PRODUCTION">In Production</option>
                                        <option value="SHIPPED">Shipped</option>
                                        <option value="COMPLETED">Completed</option>
                                    </select>
                                </td>
                            )}
                        </tr>
                    ))
                )}
                </tbody>
            </table>
            {/* 简单的CSS样式，可以移到 index.css */}
            <style jsx>{`
                th, td {
                    text-align: left;
                    padding: 12px;
                    border-bottom: 1px solid #ddd;
                }
                th {
                    background-color: #f8f9fa;
                }
                select {
                    padding: 0.5rem;
                    border-radius: 4px;
                }
            `}</style>
        </div>
    );
}

export default OrdersPage;