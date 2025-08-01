// src/components/NotificationBell.jsx

import React from 'react';
import { Indicator, Popover, Text, ScrollArea, Group, Button, ActionIcon, Tooltip } from '@mantine/core';
import { IconBell, IconMailOpened } from '@tabler/icons-react';
import { useNotifications } from '../context/NotificationContext';
import { Link } from 'react-router-dom';

function NotificationBell() {
    // 从Context获取所需的状态和函数
    const { notifications, unreadCount, markOneAsRead, markAllAsRead } = useNotifications();

    return (
        <Popover width={350} position="bottom-end" withArrow shadow="md">
            <Popover.Target>
                <Indicator label={unreadCount} size={16} disabled={unreadCount === 0} withBorder>
                    <ActionIcon variant="default" radius="xl" size="lg">
                        <IconBell size={20} />
                    </ActionIcon>
                </Indicator>
            </Popover.Target>

            <Popover.Dropdown>
                <Group position="apart" p="xs">
                    <Text fw={500}>Notifications</Text>
                    <Tooltip label="Mark all as read">
                        <ActionIcon onClick={markAllAsRead} disabled={unreadCount === 0}>
                            <IconMailOpened size={18} />
                        </ActionIcon>
                    </Tooltip>
                </Group>

                <ScrollArea h={300}>
                    {notifications.length > 0 ? (
                        notifications.map((notification) => (
                            <Link
                                to={notification.link_url || '#'}
                                key={notification.id}
                                style={{textDecoration: 'none', color: 'inherit'}}
                                // 【关键交互】点击时，调用标记已读的函数
                                onClick={() => markOneAsRead(notification.id)}
                            >
                                <div
                                    style={{
                                        padding: '0.75rem',
                                        borderTop: '1px solid #eee',
                                        // 【关键UI】根据已读状态显示不同样式
                                        backgroundColor: notification.is_read ? 'transparent' : '#e7f5ff',
                                        opacity: notification.is_read ? 0.6 : 1,
                                    }}
                                >
                                    <Text size="sm">{notification.message}</Text>
                                    <Text size="xs" color="dimmed">{new Date(notification.created_at).toLocaleString()}</Text>
                                </div>
                            </Link>
                        ))
                    ) : (
                        <Text p="md" align="center" color="dimmed">You have no notifications yet.</Text>
                    )}
                </ScrollArea>
            </Popover.Dropdown>
        </Popover>
    );
}

export default NotificationBell;