// src/pages/AdminPage.jsx
import React, { useState, useEffect, useCallback } from 'react';
import { Table, Title, Button, Badge, Tabs, Switch } from '@mantine/core';
import * as api from '../api';
import { Link } from 'react-router-dom';

function CompanyManagementTab() {
    const [companies, setCompanies] = useState([]);
    const fetchCompanies = useCallback(async () => {
        try {
            const response = await api.getAllCompanies();
            setCompanies(response.data);
        } catch (error) { console.error("Failed to fetch companies", error); }
    }, []);

    useEffect(() => { fetchCompanies(); }, [fetchCompanies]);

    const handleVerify = async (companyId) => {
        if (window.confirm("Are you sure?")) {
            await api.verifyCompany(companyId);
            fetchCompanies();
        }
    };

    const rows = companies.map((company) => (
        <Table.Tr key={company.id}>
            <Table.Td>{company.id}</Table.Td>
            <Table.Td><Link to={`/companies/${company.id}`}>{company.name}</Link></Table.Td>
            <Table.Td>{company.company_type}</Table.Td>
            <Table.Td>
                {company.is_verified ? <Badge color="teal">Verified</Badge> : <Badge color="gray">Not Verified</Badge>}
            </Table.Td>
            <Table.Td>
                {!company.is_verified && <Button size="xs" onClick={() => handleVerify(company.id)}>Verify</Button>}
            </Table.Td>
        </Table.Tr>
    ));

    return (
        <Table striped highlightOnHover withBorder mt="lg">
            <Table.Thead><Table.Tr><Table.Th>ID</Table.Th><Table.Th>Name</Table.Th><Table.Th>Type</Table.Th><Table.Th>Status</Table.Th><Table.Th>Actions</Table.Th></Table.Tr></Table.Thead>
            <Table.Tbody>{rows}</Table.Tbody>
        </Table>
    );
}

function UserManagementTab() {
    const [users, setUsers] = useState([]);
    const fetchUsers = useCallback(async () => {
        try {
            const response = await api.getAllUsers();
            setUsers(response.data);
        } catch (error) { console.error("Failed to fetch users", error); }
    }, []);

    useEffect(() => { fetchUsers(); }, [fetchUsers]);

    const handleStatusToggle = async (userId, currentStatus) => {
        await api.updateUserStatus(userId, !currentStatus);
        fetchUsers();
    };

    // 我们需要从数据库获取用户的激活状态，这里先假设一个字段 `is_active`
    // 等待后端完成后，需要确保 UserProfileResponse 包含 is_active
    // [后端已更新] - 现在我们需要更新前端的 UserProfileResponse 模型

    const rows = users.map((user) => (
        <Table.Tr key={user.id}>
            <Table.Td>{user.id}</Table.Td>
            <Table.Td>{user.full_name}</Table.Td>
            <Table.Td>{user.email}</Table.Td>
            <Table.Td><Link to={`/companies/${user.company_id}`}>{user.company_name}</Link></Table.Td>
            <Table.Td>
                <Switch
                    checked={user.is_active}
                    onChange={() => handleStatusToggle(user.id, user.is_active)}
                    label={user.is_active ? "Active" : "Disabled"}
                />
            </Table.Td>
        </Table.Tr>
    ));

    return (
        <Table striped highlightOnHover withBorder mt="lg">
            <Table.Thead><Table.Tr><Table.Th>ID</Table.Th><Table.Th>Full Name</Table.Th><Table.Th>Email</Table.Th><Table.Th>Company</Table.Th><Table.Th>Status</Table.Th></Table.Tr></Table.Thead>
            <Table.Tbody>{rows}</Table.Tbody>
        </Table>
    );
}
// [后端已更新] - UserProfileResponse 不含 is_active, 我们需要更新它
// 在 `admin_service.rs` 的 `list_all_users` 中添加 `u.is_active`
// 并在 `UserProfileResponse` struct 中添加 `pub is_active: bool,`


function AdminPage() {
    return (
        <div>
            <Title order={2} mb="lg">Admin Dashboard</Title>
            <Tabs defaultValue="companies">
                <Tabs.List>
                    <Tabs.Tab value="companies">Company Management</Tabs.Tab>
                    <Tabs.Tab value="users">User Management</Tabs.Tab>
                </Tabs.List>

                <Tabs.Panel value="companies" pt="xs">
                    <CompanyManagementTab />
                </Tabs.Panel>

                <Tabs.Panel value="users" pt="xs">
                    <UserManagementTab />
                </Tabs.Panel>
            </Tabs>
        </div>
    );
}

export default AdminPage;