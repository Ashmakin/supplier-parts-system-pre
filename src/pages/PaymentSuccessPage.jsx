// src/pages/PaymentSuccessPage.jsx

import React, { useEffect } from 'react';
import { Link } from 'react-router-dom';
import { Title, Text, Button, Container, Paper } from '@mantine/core';
import { IconCircleCheck } from '@tabler/icons-react';

function PaymentSuccessPage() {
    // 您可以在这里添加一个useEffect来从URL参数中获取session_id并发送给后端进行最终确认
    // 但核心确认逻辑依赖于Webhook，所以这里只显示成功信息
    useEffect(() => {
        // 比如：const sessionId = new URLSearchParams(window.location.search).get('session_id');
        // api.confirmPayment(sessionId);
    }, []);

    return (
        <Container size="sm" mt={50}>
            <Paper withBorder shadow="md" p={30} radius="md" style={{ textAlign: 'center' }}>
                <IconCircleCheck size={80} color="teal" style={{ margin: 'auto' }} />
                <Title order={2} mt="lg">Payment Successful!</Title>
                <Text color="dimmed" mt="sm">
                    Thank you for your payment. Your order is now being processed. You can view the updated status in your orders list.
                </Text>
                <Button component={Link} to="/orders" mt="xl" size="md">
                    View Your Orders
                </Button>
            </Paper>
        </Container>
    );
}

export default PaymentSuccessPage;