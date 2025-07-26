// src/components/BuyerAnalytics.jsx

import React, { useState, useEffect } from 'react';
import { Chart as ChartJS, ArcElement, Tooltip, Legend, CategoryScale, LinearScale, BarElement, Title } from 'chart.js';
import { Pie } from 'react-chartjs-2';
import * as api from '../api';

ChartJS.register(ArcElement, Tooltip, Legend, CategoryScale, LinearScale, BarElement, Title);

function BuyerAnalytics() {
    const [stats, setStats] = useState(null);
    const [spendingData, setSpendingData] = useState(null);
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        const fetchData = async () => {
            try {
                const [statsRes, spendingRes] = await Promise.all([
                    api.getBuyerStats(),
                    api.getSpendingBySupplier()
                ]);
                setStats(statsRes.data);

                // 准备图表数据
                const labels = spendingRes.data.map(item => item.supplier_name);
                const data = spendingRes.data.map(item => item.total);
                setSpendingData({
                    labels,
                    datasets: [{
                        label: 'Spending',
                        data,
                        backgroundColor: [
                            'rgba(255, 99, 132, 0.6)',
                            'rgba(54, 162, 235, 0.6)',
                            'rgba(255, 206, 86, 0.6)',
                            'rgba(75, 192, 192, 0.6)',
                            'rgba(153, 102, 255, 0.6)',
                            'rgba(255, 159, 64, 0.6)',
                        ],
                        borderColor: [
                            'rgba(255, 99, 132, 1)',
                            'rgba(54, 162, 235, 1)',
                            'rgba(255, 206, 86, 1)',
                            'rgba(75, 192, 192, 1)',
                            'rgba(153, 102, 255, 1)',
                            'rgba(255, 159, 64, 1)',
                        ],
                        borderWidth: 1,
                    }]
                });

            } catch (error) {
                console.error("Failed to fetch analytics data", error);
            } finally {
                setIsLoading(false);
            }
        };
        fetchData();
    }, []);

    if (isLoading) return <p>Loading analytics...</p>;

    return (
        <div style={{ background: '#fff', padding: '2rem', borderRadius: 'var(--border-radius)', boxShadow: '0 2px 4px rgba(0,0,0,0.05)' }}>
            <h3>Procurement Analytics</h3>
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '1rem', marginBottom: '2rem' }}>
                <div style={{ padding: '1rem', border: '1px solid #eee', borderRadius: 'var(--border-radius)' }}>
                    <h4>Total Orders</h4>
                    <p style={{ fontSize: '2rem', fontWeight: 'bold' }}>{stats?.total_orders || 0}</p>
                </div>
                <div style={{ padding: '1rem', border: '1px solid #eee', borderRadius: 'var(--border-radius)' }}>
                    <h4>Total Spent</h4>
                    <p style={{ fontSize: '2rem', fontWeight: 'bold' }}>${parseFloat(stats?.total_spent || 0).toFixed(2)}</p>
                </div>
                <div style={{ padding: '1rem', border: '1px solid #eee', borderRadius: 'var(--border-radius)' }}>
                    <h4>Suppliers Worked With</h4>
                    <p style={{ fontSize: '2rem', fontWeight: 'bold' }}>{stats?.distinct_suppliers || 0}</p>
                </div>
            </div>

            <h4>Spending by Supplier</h4>
            {spendingData && spendingData.labels.length > 0 ? (
                <div style={{ maxWidth: '400px', margin: 'auto' }}>
                    <Pie data={spendingData} />
                </div>
            ) : (
                <p>No spending data to display yet.</p>
            )}
        </div>
    );
}

export default BuyerAnalytics;