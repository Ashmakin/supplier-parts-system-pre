import React from 'react';
import { Link } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';

// 导入所有需要的Mantine组件
import {
    Container,
    Title,
    Text,
    Button,
    Group,
    SimpleGrid,
    Paper,
    ThemeIcon,
    Divider,
} from '@mantine/core';

// 导入一些图标来丰富视觉效果
import { IconFileText, IconMessages, IconChartPie } from '@tabler/icons-react';

// 一个可复用的“功能特性”卡片组件
function FeatureCard({ icon, title, description }) {
    return (
        <Paper withBorder radius="md" p="lg">
            <Group>
                <ThemeIcon size="xl" radius="md" variant="light">
                    {icon}
                </ThemeIcon>
                <Text fw={700} size="lg">{title}</Text>
            </Group>
            <Text size="sm" c="dimmed" mt="sm">
                {description}
            </Text>
        </Paper>
    );
}


function HomePage() {
    const { user } = useAuth(); // 获取当前用户登录状态

    return (
        <Container size="lg" py="xl">
            {/* Hero Section - 核心宣传区域 */}
            <Paper style={{ textAlign: 'center', padding: '4rem 1rem', backgroundColor: 'transparent' }}>
                <Title order={1} style={{ fontSize: '3rem' }}>
                    Intelligent Supply Chain,
                    <Text
                        component="span"
                        variant="gradient"
                        gradient={{ from: 'indigo', to: 'cyan' }}
                        inherit
                        ml="sm"
                    >
                        Seamless Collaboration
                    </Text>
                </Title>

                <Text c="dimmed" mt="lg" size="xl" maw={600} mx="auto">
                    Connect with verified suppliers, manage your RFQs, and streamline your procurement process from start to finish on a single, powerful platform.
                </Text>

                {/* Call-to-Action Buttons - 根据登录状态显示不同按钮 */}
                <Group justify="center" mt="xl">
                    {user ? (
                        <Button component={Link} to="/dashboard" size="lg">
                            Go to Your Dashboard
                        </Button>
                    ) : (
                        <>
                            <Button component={Link} to="/register" size="lg">
                                Get Started
                            </Button>
                            <Button component={Link} to="/login" variant="default" size="lg">
                                Sign In
                            </Button>
                        </>
                    )}
                </Group>
            </Paper>

            <Divider my="xl" label="Platform Features" labelPosition="center" />

            {/* Features Section - 功能特性展示区 */}
            <SimpleGrid cols={{ base: 1, sm: 2, lg: 3 }} spacing="lg">
                <FeatureCard
                    icon={<IconFileText size={28} />}
                    title="Streamlined RFQ Process"
                    description="Easily create and manage Requests for Quotation with file attachments. Receive and compare quotes from multiple suppliers in one place."
                />
                <FeatureCard
                    icon={<IconMessages size={28} />}
                    title="Real-time Collaboration"
                    description="Communicate with suppliers directly within the context of an RFQ using our real-time chat, ensuring all clarifications are documented."
                />
                <FeatureCard
                    icon={<IconChartPie size={28} />}
                    title="Data-Driven Insights"
                    description="Leverage analytics dashboards to track your spending, evaluate supplier performance, and make smarter procurement decisions."
                />
            </SimpleGrid>
        </Container>
    );
}

export default HomePage;