// CompanyProfilePage.jsx
import React, { useState, useEffect, useCallback, useMemo } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import * as api from '../api';

import {
    Paper,
    Title,
    Text,
    Button,
    Badge,
    Group,
    MultiSelect,
    Textarea,
    Loader,
    Center,
    Alert,
} from '@mantine/core';
import { IconAlertCircle } from '@tabler/icons-react';

/** 编辑公司简介 */
function EditDescription({ initialDescription, onSave, onCancel }) {
    const [description, setDescription] = useState(initialDescription);
    const [isSaving, setIsSaving] = useState(false);

    const handleSave = async (e) => {
        e.preventDefault();
        setIsSaving(true);
        try {
            await onSave(description);
        } catch (err) {
            console.error('保存简介失败：', err);
        } finally {
            setIsSaving(false);
        }
    };

    return (
        <Paper withBorder p="md" mt="lg" radius="md">
            <form onSubmit={handleSave}>
                <Textarea
                    label="Edit your company description"
                    autosize
                    minRows={5}
                    value={description}
                    onChange={(e) => setDescription(e.currentTarget.value)}
                />
                <Group position="right" mt="md">
                    <Button variant="default" onClick={onCancel} disabled={isSaving}>
                        Cancel
                    </Button>
                    <Button type="submit" loading={isSaving}>
                        Save Changes
                    </Button>
                </Group>
            </form>
        </Paper>
    );
}

/** 编辑能力标签 */
function EditCapabilities({
                              companyCapabilities = [],
                              allCapabilities = [],
                              onSave,   // 接收选中的 ID 数组
                              onCancel,
                          }) {
    const [selectedIds, setSelectedIds] = useState(
        companyCapabilities.map((c) => String(c.id))
    );
    const [isSaving, setIsSaving] = useState(false);

    // 扁平化 data
    const dataForMultiSelect = useMemo(() => {
        if (!Array.isArray(allCapabilities)) return [];
        return allCapabilities.map((c) => ({
            value: String(c.id),
            label: c.name,
        }));
    }, [allCapabilities]);

    const handleSave = async () => {
        setIsSaving(true);
        try {
            await onSave(selectedIds);
            onCancel();
        } catch (err) {
            console.error('保存能力失败：', err);
        } finally {
            setIsSaving(false);
        }
    };

    return (
        <Paper withBorder p="md" mt="lg" radius="md">
            <MultiSelect
                data={dataForMultiSelect}
                value={selectedIds}
                onChange={setSelectedIds}
                label="Select your company's capabilities"
                placeholder="Click to select"
                searchable
                clearable
                loading={isSaving}
            />
            <Group position="right" mt="md">
                <Button variant="subtle" onClick={onCancel} disabled={isSaving}>
                    Cancel
                </Button>
                <Button onClick={handleSave} loading={isSaving} disabled={isSaving}>
                    Save Capabilities
                </Button>
            </Group>
        </Paper>
    );
}

/** 主页面 */
export default function CompanyProfilePage() {
    const { companyId } = useParams();
    const { user } = useAuth();

    const [profile, setProfile] = useState(null);
    const [capabilities, setCapabilities] = useState([]);
    const [allCapabilities, setAllCapabilities] = useState([]);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState('');

    const [isEditingDesc, setIsEditingDesc] = useState(false);
    const [isEditingCaps, setIsEditingCaps] = useState(false);

    // 拉取数据
    const fetchProfileData = useCallback(async () => {
        setIsLoading(true);
        setError('');
        try {
            const [profileRes, capsRes, allCapsRes] = await Promise.all([
                api.getCompanyProfile(companyId),
                api.getCompanyCapabilities(companyId),
                api.getAllCapabilities(),
            ]);
            setProfile(profileRes.data);
            setCapabilities(Array.isArray(capsRes.data) ? capsRes.data : []);
            setAllCapabilities(Array.isArray(allCapsRes.data) ? allCapsRes.data : []);
        } catch (err) {
            console.error('加载失败：', err);
            setError('Could not load company profile. Please try again later.');
            setCapabilities([]);
            setAllCapabilities([]);
        } finally {
            setIsLoading(false);
        }
    }, [companyId]);

    useEffect(() => {
        fetchProfileData();
    }, [fetchProfileData]);

    // 更新简介
    const handleUpdateDescription = async (newDescription) => {
        await api.updateCompanyProfile(companyId, { description: newDescription });
        setIsEditingDesc(false);
        fetchProfileData();
    };

    // 更新能力：调用新增/删除接口
    const handleSaveCapabilities = async (selectedIds) => {
        // 当前已有 ID 列表
        const oldIds = capabilities.map((c) => String(c.id));
        // 计算新增与删除
        const toAdd = selectedIds.filter((id) => !oldIds.includes(id));
        const toRemove = oldIds.filter((id) => !selectedIds.includes(id));

        // 并行调用
        await Promise.all([
            ...toAdd.map((id) => api.addCapabilityToCompany(Number(id))),
            ...toRemove.map((id) => api.removeCapabilityFromCompany(id)),
        ]);

        // 重新拉取并结束编辑
        fetchProfileData();
    };

    const isOwner = user && user.company_id === parseInt(companyId, 10);

    if (isLoading) {
        return (
            <Center style={{ height: '80vh' }}>
                <Loader />
            </Center>
        );
    }
    if (error) {
        return (
            <Alert icon={<IconAlertCircle size="1rem" />} title="Error" color="red">
                {error}
            </Alert>
        );
    }
    if (!profile) {
        return <div className="container">Company not found.</div>;
    }

    return (
        <div>
            <Button component={Link} to="/dashboard" variant="subtle" mb="md">
                &larr; Back to Dashboard
            </Button>

            <Paper withBorder p="xl" radius="md">
                <Group position="apart" align="flex-start">
                    <div>
                        <Title order={2}>{profile.name}</Title>
                        <Text color="dimmed">
                            {profile.company_type} based in {profile.city || 'N/A'}
                        </Text>
                        {profile.is_verified && (
                            <Badge color="teal" variant="light" mt="sm">
                                Verified
                            </Badge>
                        )}
                    </div>
                </Group>

                <hr style={{ margin: '2rem 0' }} />

                {/* 能力展示 */}
                <Title order={4}>Capabilities</Title>
                <Group mt="sm" mb="md">
                    {capabilities.length > 0 ? (
                        capabilities.map((cap) => (
                            <Badge key={cap.id} size="lg">
                                {cap.name}
                            </Badge>
                        ))
                    ) : (
                        <Text color="dimmed" size="sm">
                            No capabilities listed yet.
                        </Text>
                    )}
                </Group>
                {isOwner && profile.company_type === 'SUPPLIER' && !isEditingCaps && (
                    <Button variant="light" size="xs" onClick={() => setIsEditingCaps(true)}>
                        Edit Capabilities
                    </Button>
                )}
                {isEditingCaps && (
                    <EditCapabilities
                        companyCapabilities={capabilities}
                        allCapabilities={allCapabilities}
                        onSave={handleSaveCapabilities}
                        onCancel={() => setIsEditingCaps(false)}
                    />
                )}

                <hr style={{ margin: '2rem 0' }} />

                {/* 公司简介 */}
                <Title order={4}>About Us</Title>
                <Text mt="sm" mb="md" style={{ whiteSpace: 'pre-wrap' }}>
                    {profile.description || 'No description provided.'}
                </Text>
                {isOwner && !isEditingDesc && (
                    <Button onClick={() => setIsEditingDesc(true)} variant="light" size="xs">
                        Edit Description
                    </Button>
                )}
                {isEditingDesc && (
                    <EditDescription
                        initialDescription={profile.description || ''}
                        onSave={handleUpdateDescription}
                        onCancel={() => setIsEditingDesc(false)}
                    />
                )}
            </Paper>
        </div>
    );
}