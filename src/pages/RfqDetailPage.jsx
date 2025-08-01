import {React, useState, useEffect, useCallback } from 'react';
import {useParams, Link, useNavigate} from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import * as api from '../api';
import ChatBox from '../components/ChatBox'; // 确保ChatBox组件已导入

// 导入所有需要的Mantine组件
import {
    Container,
    Title,
    Text,
    Button,
    Paper,
    Group,
    Badge,
    Grid,
    Stack,
    Alert,
    Loader,
    Center,
    Card,
    NumberInput,
    Textarea,
    List,
    ThemeIcon, Divider, SimpleGrid,
} from '@mantine/core';
import { IconAlertCircle, IconCircleCheck, IconFile } from '@tabler/icons-react';

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
            onQuoteSubmitted();
        } catch (error) {
            console.error('Failed to submit quote', error);
            alert('Failed to submit quote.');
        } finally {
            setIsSubmitting(false);
        }
    };

    return (
        <Paper withBorder p="xl" radius="md">
            <Title order={4} mb="lg">Submit Your Quote</Title>
            <form onSubmit={handleSubmit}>
                <Stack>
                    <NumberInput
                        label="Your Price (per unit)"
                        placeholder="10.50"
                        precision={2}
                        value={price}
                        onChange={setPrice}
                        required
                    />
                    <NumberInput
                        label="Your Lead Time (in days)"
                        placeholder="15"
                        value={lead_time_days}
                        onChange={setLeadTime}
                        required
                    />
                    <Textarea
                        label="Notes (optional)"
                        placeholder="Any additional details..."
                        value={notes}
                        onChange={(event) => setNotes(event.currentTarget.value)}
                        autosize
                        minRows={2}
                    />
                    <Button type="submit" mt="md" loading={isSubmitting}>
                        Submit Quote
                    </Button>
                </Stack>
            </form>
        </Paper>
    );
}

/**
 * 采购方看到的报价列表
 */
function QuoteList({ quotes, onAccept, rfqStatus }) {
    if (!quotes.length) return <Text c="dimmed" ta="center" mt="xl">No quotes have been received yet.</Text>;

    return (
        <Stack>
            <Title order={4}>Received Quotes</Title>
            {quotes.map(quote => (
                <Card withBorder p="md" radius="md" key={quote.id}>
                    <Group position="apart">
                        <Text fw={500}>{quote.supplier_company_name}</Text>
                        <Badge color={quote.status === 'ACCEPTED' ? 'teal' : 'gray'}>{quote.status}</Badge>
                    </Group>
                    <Text size="xl" fw={700} mt="sm">${parseFloat(quote.total || quote.price).toLocaleString()}</Text>
                    <Text size="sm" c="dimmed">{quote.lead_time_days} days lead time</Text>
                    <Text size="sm" mt="xs">{quote.notes || 'No additional notes.'}</Text>
                    {rfqStatus === 'OPEN' && (
                        <Button onClick={() => onAccept(quote.id)} mt="md" fullWidth variant="light">
                            Accept Quote
                        </Button>
                    )}
                </Card>
            ))}
        </Stack>
    );
}

/**
 * 主详情页组件
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
            const rfqResponse = await api.getRfqById(rfqId);
            const fetchedRfq = rfqResponse.data;
            setRfq(fetchedRfq);

            const attachmentsResponse = await api.getRfqAttachments(rfqId);
            setAttachments(attachmentsResponse.data);

            if (user.company_type === 'BUYER' && user.company_id === fetchedRfq.buyer_company_id) {
                const quotesResponse = await api.getQuotesForRfq(rfqId);
                setQuotes(quotesResponse.data);
            }
        } catch (err) {
            console.error("Failed to fetch RFQ details", err);
            setError('Failed to load RFQ data.');
        } finally {
            setIsLoading(false);
        }
    }, [rfqId, user]);

    useEffect(() => {
        fetchData();
    }, [fetchData]);

    const handleAcceptQuote = async (quoteId) => {
        if (window.confirm("Are you sure? This will create a purchase order and close the RFQ.")) {
            try {
                await api.acceptQuote(quoteId);
                alert("Quote accepted! A purchase order has been created.");
                navigate('/orders'); // 接受后直接跳转到订单页
            } catch (error) {
                console.error("Failed to accept quote", error);
                alert("Failed to accept quote.");
            }
        }
    };

    if (isLoading) return <Center style={{ height: '80vh' }}><Loader /></Center>;
    if (error) return <Alert icon={<IconAlertCircle size="1rem" />} title="Error" color="red">{error}</Alert>;
    if (!rfq) return <div className="container"><h2>RFQ not found.</h2></div>;

    const isOwner = user.company_type === 'BUYER' && user.company_id === rfq.buyer_company_id;
    const canSupplierQuote = user.company_type === 'SUPPLIER' && rfq.status === 'OPEN';

    return (
        <Container size="lg">
            <Button component={Link} to="/dashboard" variant="subtle" mb="md" pl={0}>&larr; Back to Dashboard</Button>

            <Grid>
                {/* 左侧信息栏 */}
                <Grid.Col span={{ base: 12, md: 7 }}>
                    <Paper withBorder p="xl" radius="md">
                        <Group position="apart" align="flex-start">
                            <div>
                                <Title order={2}>{rfq.title}</Title>
                                <Text c="dimmed">
                                    Posted by <Link to={`/companies/${rfq.buyer_company_id}`}>{rfq.buyer_company_name}</Link>
                                </Text>
                            </div>
                            <Badge size="lg" variant="filled" color={rfq.status === 'OPEN' ? 'blue' : 'gray'}>
                                {rfq.status}
                            </Badge>
                        </Group>

                        <Divider my="lg" />

                        <SimpleGrid cols={2}>
                            <div>
                                <Text size="sm" c="dimmed">Quantity</Text>
                                <Text fw={500}>{rfq.quantity}</Text>
                            </div>
                            <div>
                                <Text size="sm" c="dimmed">Created At</Text>
                                <Text fw={500}>{new Date(rfq.created_at).toLocaleDateString()}</Text>
                            </div>
                        </SimpleGrid>

                        <Divider my="lg" />

                        <Title order={4}>Description</Title>
                        <Text mt="sm" mb="md" style={{whiteSpace: 'pre-wrap'}}>
                            {rfq.description || "No description provided."}
                        </Text>

                        <Title order={4}>Attachments</Title>
                        {attachments.length > 0 ? (
                            <List
                                spacing="xs"
                                size="sm"
                                center
                                icon={<ThemeIcon color="gray" size={24} radius="xl"><IconFile size={16} /></ThemeIcon>}
                                mt="sm"
                            >
                                {attachments.map(att => (
                                    <List.Item key={att.id}>
                                        <a href={`http://127.0.0.1:8080${att.stored_path.replace('./', '/')}`} target="_blank" rel="noopener noreferrer">
                                            {att.original_filename}
                                        </a>
                                    </List.Item>
                                ))}
                            </List>
                        ) : (
                            <Text c="dimmed" size="sm" mt="sm">No attachments.</Text>
                        )}
                    </Paper>
                </Grid.Col>

                {/* 右侧操作栏 */}
                <Grid.Col span={{ base: 12, md: 5 }}>
                    <Stack>
                        {isOwner && <QuoteList quotes={quotes} onAccept={handleAcceptQuote} rfqStatus={rfq.status} />}
                        {canSupplierQuote && <CreateQuoteForm rfqId={rfq.id} onQuoteSubmitted={fetchData} />}
                        {rfq.status === 'AWARDED' && (
                            <Alert icon={<IconCircleCheck size="1rem" />} title="RFQ Awarded" color="teal">
                                This RFQ has been awarded and is closed for new quotes. A purchase order has been created.
                            </Alert>
                        )}
                        <ChatBox rfqId={rfqId} />
                    </Stack>
                </Grid.Col>
            </Grid>
        </Container>
    );
}

export default RfqDetailPage;