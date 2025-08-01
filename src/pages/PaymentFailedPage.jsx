import React from 'react';
import { Link } from 'react-router-dom';
import { Title, Text, Button, Container } from '@mantine/core';

function PaymentFailedPage() {
    return (
        <Container>
            <Title order={1}>Payment Failed!</Title>
            <Text>Your Payment is canceled for some error.</Text>
            <Button component={Link} to="/orders" mt="md">
                View Your Orders
            </Button>
        </Container>
    );
}

export default PaymentFailedPage;