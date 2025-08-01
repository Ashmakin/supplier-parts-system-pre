import React, { useState, useEffect, useCallback } from 'react';
import { useAuth } from '../context/AuthContext';
import * as api from '../api';
import { Link } from 'react-router-dom';

// 导入所有需要的Mantine组件
import {
    Title,
    Text,
    Button,
    Paper,
    SimpleGrid,
    Card,
    Table,
    Grid,
    Stack,
    TextInput,
    Textarea,
    NumberInput,
    FileInput,
} from '@mantine/core';

// 导入我们已有的分析组件
import BuyerAnalytics from '../components/BuyerAnalytics';
import SupplierAnalytics from '../components/SupplierAnalytics';

/**
 * 创建RFQ的表单子组件
 */
function CreateRfqForm({ onRfqCreated }) {
    const [title, setTitle] = useState('');
    const [description, setDescription] = useState('');
    const [quantity, setQuantity] = useState(1);
    const [attachment, setAttachment] = useState(null);
    const [isSubmitting, setIsSubmitting] = useState(false);

    const handleSubmit = async (e) => {
        e.preventDefault();
        setIsSubmitting(true);

        const formData = new FormData();
        formData.append('title', title);
        formData.append('description', description);
        formData.append('quantity', Number(quantity) || 1);
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
            e.target.reset();
            onRfqCreated();
        } catch (error) {
            console.error("Failed to create RFQ:", error);
            alert('Failed to create RFQ. Check the console for details.');
        } finally {
            setIsSubmitting(false);
        }
    };

    return (
        <Paper withBorder p="xl" radius="md">
            <Title order={3} mb="lg">Create New RFQ</Title>
            <form onSubmit={handleSubmit}>
                <Stack>
                    <TextInput
                        label="Title"
                        placeholder="e.g., Custom Aluminum Part"
                        required
                        value={title}
                        onChange={(event) => setTitle(event.currentTarget.value)}
                    />
                    <Textarea
                        label="Description / Specs"
                        placeholder="Details about materials, tolerances, etc."
                        value={description}
                        onChange={(event) => setDescription(event.currentTarget.value)}
                    />
                    <NumberInput
                        label="Quantity"
                        placeholder="1000"
                        required
                        min={1}
                        value={quantity}
                        onChange={(value) => setQuantity(value || 1)}
                    />
                    <FileInput
                        label="Attachment (PDF, CAD, etc.)"
                        placeholder="Upload file"
                        value={attachment}
                        onChange={setAttachment}
                        clearable
                    />
                    <Button type="submit" mt="md" loading={isSubmitting}>
                        Submit RFQ
                    </Button>
                </Stack>
            </form>
        </Paper>
    );
}

/**
 * 搜索和筛选表单子组件
 */
function FilterForm({ onSearch }) {
    const [searchTerm, setSearchTerm] = useState('');
    const [city, setCity] = useState('');

    const handleSearch = (e) => {
        e.preventDefault();
        onSearch({ search: searchTerm, city });
    };

    return (
        <Paper withBorder p="md" radius="md" mb="xl">
            <form onSubmit={handleSearch}>
                <SimpleGrid cols={{ base: 1, md: 3 }} spacing="md">
                    <TextInput
                        placeholder="Search by keyword..."
                        value={searchTerm}
                        onChange={e => setSearchTerm(e.target.value)}
                    />
                    <TextInput
                        placeholder="Filter by city..."
                        value={city}
                        onChange={e => setCity(e.target.value)}
                    />
                    <Button type="submit">Search</Button>
                </SimpleGrid>
            </form>
        </Paper>
    );
}

/**
 * RFQ列表子组件
 */
function RfqList({ rfqs }) {
    if (!rfqs || rfqs.length === 0) {
        return <Text mt="md">No open RFQs found with the current filters.</Text>;
    }

    const rows = rfqs.map(rfq => (
        <Table.Tr key={rfq.id}>
            <Table.Td>
                <Text fw={500}>{rfq.title}</Text>
                <Text size="xs" c="dimmed">{rfq.quantity} pcs</Text>
            </Table.Td>
            <Table.Td>{rfq.buyer_company_name}</Table.Td>
            <Table.Td>{rfq.city || 'N/A'}</Table.Td>
            <Table.Td>{rfq.status}</Table.Td>
            <Table.Td style={{ textAlign: 'right' }}>
                <Button component={Link} to={`/rfqs/${rfq.id}`} variant="light" size="xs">
                    View Details
                </Button>
            </Table.Td>
        </Table.Tr>
    ));

    return (
        <Card withBorder p={0} radius="md">
            <Table striped highlightOnHover verticalSpacing="sm">
                <Table.Thead>
                    <Table.Tr>
                        <Table.Th>RFQ Title</Table.Th>
                        <Table.Th>Buyer</Table.Th>
                        <Table.Th>City</Table.Th>
                        <Table.Th>Status</Table.Th>
                        <Table.Th />
                    </Table.Tr>
                </Table.Thead>
                <Table.Tbody>{rows}</Table.Tbody>
            </Table>
        </Card>
    );
}


/**
 * 主仪表盘页面组件
 */
function DashboardPage() {
    const { user } = useAuth();
    const [rfqs, setRfqs] = useState([]);
    const [isLoading, setIsLoading] = useState(true);
    const [filters, setFilters] = useState({ search: '', city: '' });

    const fetchRfqs = useCallback(async () => {
        if (!user) return;
        setIsLoading(true);
        try {
            const response = await api.getRfqs(filters);
            setRfqs(response.data);
        } catch (error) {
            console.error("Failed to fetch RFQs", error);
            setRfqs([]);
        } finally {
            setIsLoading(false);
        }
    }, [user, filters]);

    useEffect(() => {
        fetchRfqs();
    }, [fetchRfqs]);

    const handleSearch = (newFilters) => {
        setFilters(newFilters);
    };

    if (!user) return <div className="container">Loading user data...</div>;

    return (
        <div>
            <Title order={1} mb="xs">Dashboard</Title>
            <Text mb="xl" c="dimmed">Welcome back, here's an overview of your activities.</Text>

            <Grid>
                {/* 主内容区 (左侧) */}
                <Grid.Col span={{ base: 12, lg: 8 }}>
                    <Stack>
                        <Title order={3}>{user.company_type === 'BUYER' ? "My Open RFQs" : "Find Open RFQs"}</Title>
                        {user.company_type === 'SUPPLIER' && <FilterForm onSearch={handleSearch} />}
                        {isLoading ? <p>Loading RFQs...</p> : <RfqList rfqs={rfqs} />}
                    </Stack>
                </Grid.Col>

                {/* 侧边栏区 (右侧) */}
                <Grid.Col span={{ base: 12, lg: 4 }}>
                    <Stack>
                        {user.company_type === 'BUYER' && <BuyerAnalytics />}
                        {user.company_type === 'SUPPLIER' && <SupplierAnalytics />}
                        {user.company_type === 'BUYER' && <CreateRfqForm onRfqCreated={fetchRfqs} />}
                    </Stack>
                </Grid.Col>
            </Grid>
        </div>
    );
}

export default DashboardPage;