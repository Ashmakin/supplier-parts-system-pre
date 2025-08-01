import React, { useState, useEffect, useCallback } from 'react';
import { useAuth } from '../context/AuthContext';
import * as api from '../api';
import { useStripe } from '@stripe/react-stripe-js';

// 导入所有需要的Mantine组件
import {
    Table,
    Select,
    Title,
    Text,
    Paper,
    Alert,
    Button,
    Badge,
    Group,
    Center,
} from '@mantine/core';
import { IconAlertCircle, IconPackage } from '@tabler/icons-react';

// 一个辅助函数，根据状态返回对应的颜色
const getStatusColor = (status) => {
    switch (status) {
        case 'COMPLETED':
            return 'teal';
        case 'SHIPPED':
            return 'yellow';
        case 'IN_PRODUCTION':
            return 'blue';
        case 'PENDING_CONFIRMATION':
            return 'gray';
        default:
            return 'gray';
    }
};

const getPaymentStatusColor = (status) => {
    switch (status) {
        case 'PAID':
            return 'teal';
        case 'UNPAID':
            return 'orange';
        case 'FAILED':
            return 'red';
        default:
            return 'gray';
    }
};


function OrdersPage() {
    const { user } = useAuth();
    const stripe = useStripe();
    const [orders, setOrders] = useState([]);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState('');

    const fetchOrders = useCallback(async () => {
        setIsLoading(true);
        setError('');
        try {
            const response = await api.getOrders();
            setOrders(response.data.sort((a, b) => new Date(b.created_at) - new Date(a.created_at)));
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
            fetchOrders();
        } catch (err) {
            console.error('Failed to update status', err);
            alert('Failed to update status.');
        }
    };

    const handlePayNow = async (orderId) => {
        if (!stripe) {
            alert("Payment system is not ready, please try again in a moment.");
            return;
        }
        try {
            const response = await api.createCheckoutSession(orderId);
            const { session_id } = response.data;
            const { error } = await stripe.redirectToCheckout({ sessionId: session_id });
            if (error) {
                alert(`Payment failed: ${error.message}`);
            }
        } catch (err) {
            alert("Could not initiate payment. Please try again.");
        }
    };


    if (isLoading) return <div>Loading orders...</div>;

    const OrderStatusSelector = ({ order }) => (
        <Select
            value={order.status}
            onChange={(value) => handleStatusChange(order.id, value)}
            disabled={order.status === 'COMPLETED' || order.status === 'SHIPPED'}
            data={[
                { value: 'PENDING_CONFIRMATION', label: 'Pending Confirmation', disabled: true },
                { value: 'IN_PRODUCTION', label: 'In Production' },
                { value: 'SHIPPED', label: 'Shipped' },
                { value: 'COMPLETED', label: 'Completed' },
            ]}
        />
    );

    const rows = orders.map(order => (
        <Table.Tr key={order.id}>
            <Table.Td>#{order.id}</Table.Td>
            <Table.Td>
                <Text fw={500}>{order.rfq_title}</Text>
            </Table.Td>
            <Table.Td>
                <Text size="sm">{user.company_type === 'BUYER' ? order.supplier_name : order.buyer_name}</Text>
            </Table.Td>
            <Table.Td style={{ textAlign: 'right' }}>
                <Text fw={500}>${parseFloat(order.total_amount).toLocaleString()}</Text>
            </Table.Td>
            <Table.Td>
                <Badge color={getStatusColor(order.status)} variant="light">{order.status}</Badge>
            </Table.Td>
            <Table.Td>
                <Badge color={getPaymentStatusColor(order.payment_status)} variant="light">{order.payment_status}</Badge>
            </Table.Td>
            <Table.Td>
                {user.company_type === 'BUYER' && order.payment_status === 'UNPAID' && (
                    <Button onClick={() => handlePayNow(order.id)} size="xs">
                        Pay Now
                    </Button>
                )}
                {user.company_type === 'SUPPLIER' && <OrderStatusSelector order={order} />}
            </Table.Td>
        </Table.Tr>
    ));

    return (
        <div>
            <Title order={1} mb="xs">My Orders</Title>
            <Text mb="xl" c="dimmed">Here you can find all the purchase orders associated with your company.</Text>

            {error && (
                <Alert icon={<IconAlertCircle size="1rem" />} title="Error" color="red">
                    {error}
                </Alert>
            )}

            <Paper withBorder shadow="sm" radius="md">
                <Table.ScrollContainer minWidth={800}>
                    <Table striped highlightOnHover verticalSpacing="md">
                        <Table.Thead>
                            <Table.Tr>
                                <Table.Th>Order ID</Table.Th>
                                <Table.Th>RFQ Title</Table.Th>
                                <Table.Th>{user.company_type === 'BUYER' ? 'Supplier' : 'Buyer'}</Table.Th>
                                <Table.Th style={{ textAlign: 'right' }}>Amount</Table.Th>
                                <Table.Th>Order Status</Table.Th>
                                <Table.Th>Payment Status</Table.Th>
                                <Table.Th>Actions</Table.Th>
                            </Table.Tr>
                        </Table.Thead>
                        <Table.Tbody>
                            {rows.length > 0 ? rows : (
                                <Table.Tr>
                                    <Table.Td colSpan={7}>
                                        <Center p="xl">
                                            <Group>
                                                <IconPackage size={40} stroke={1.5} color='gray' />
                                                <Text c="dimmed">No orders found.</Text>
                                            </Group>
                                        </Center>
                                    </Table.Td>
                                </Table.Tr>
                            )}
                        </Table.Tbody>
                    </Table>
                </Table.ScrollContainer>
            </Paper>
        </div>
    );
}

export default OrdersPage;