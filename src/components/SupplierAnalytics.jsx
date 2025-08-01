// src/components/SupplierAnalytics.jsx

import React, { useState, useEffect } from 'react';
import { Card, Text, SimpleGrid, Title, Paper, Alert, Center, Loader } from '@mantine/core';
import { IconAlertCircle } from '@tabler/icons-react';
import * as api from '../api';

// 一个可复用的、用于显示单个统计指标的卡片组件
function StatCard({ title, value }) {
    return (
        <Paper withBorder p="md" radius="md">
            <Text size="xs" c="dimmed" tt="uppercase" fw={700}>
                {title}
            </Text>
            <Text size="xl" fw={700} mt={5}>
                {value}
            </Text>
        </Paper>
    );
}

function SupplierAnalytics() {
    const [stats, setStats] = useState(null);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState('');

    useEffect(() => {
        const fetchData = async () => {
            setIsLoading(true);
            try {
                const response = await api.getSupplierStats();
                setStats(response.data);
            } catch (error) {
                console.error("Failed to fetch supplier stats", error);
                setError("Could not load performance data.");
            } finally {
                setIsLoading(false);
            }
        };
        fetchData();
    }, []);

    if (isLoading) {
        return <Center p="xl"><Loader /></Center>;
    }

    if (error) {
        return <Alert icon={<IconAlertCircle size="1rem" />} title="Error" color="red">{error}</Alert>;
    }

    if (!stats) {
        return null; // 如果没有数据，不显示任何内容
    }

    // 在前端计算报价成功率，并处理除以零的情况
    const winRate = stats.total_quotes_submitted > 0
        ? ((stats.accepted_quotes / stats.total_quotes_submitted) * 100).toFixed(1) + '%'
        : '0%';

    return (
        <Paper withBorder p="xl" radius="md">
            <Title order={3} mb="lg">My Performance</Title>
            <SimpleGrid cols={3} spacing="md">
                <StatCard title="Quotes Submitted" value={stats.total_quotes_submitted} />
                <StatCard title="Quote Win Rate" value={winRate} />
                <StatCard
                    title="Revenue (Completed)"
                    value={`$${parseFloat(stats.total_revenue).toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`}
                />
            </SimpleGrid>
        </Paper>
    );
}

export default SupplierAnalytics;