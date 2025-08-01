import axios from 'axios';

// 创建一个Axios客户端实例
const apiClient = axios.create({
    // Vite 会在构建生产版本时，自动将 import.meta.env.VITE_API_BASE_URL 替换为你在.env.production中设定的值
    baseURL: `${import.meta.env.VITE_API_BASE_URL}/api`,
});

// 设置请求拦截器以附加JWT
apiClient.interceptors.request.use(
    (config) => {
        const token = localStorage.getItem('token');
        if (token) {
            config.headers.Authorization = `Bearer ${token}`;
        }
        return config;
    },
    (error) => Promise.reject(error)
);



// --- Auth API ---
export const register = (data) => apiClient.post('/auth/register', data);
export const login = (credentials) => apiClient.post('/auth/login', credentials);

// --- Admin API ---
export const getAllCompanies = () => apiClient.get('/admin/companies');
export const verifyCompany = (id) => apiClient.put(`/admin/companies/${id}/verify`);

// --- 新增 ---
export const getAllUsers = () => apiClient.get('/admin/users');
export const updateUserStatus = (id, isActive) => apiClient.put(`/admin/users/${id}/status`, { is_active: isActive });

// --- RFQ API ---
export const createRfq = (formData) => {
    // 现在这个函数将正常工作，因为 apiClient 不再强制使用 json header
    return apiClient.post('/rfqs', formData);
};

export const getRfqs = (params) => {
    return apiClient.get('/rfqs', { params });
};
export const getRfqById = (id) => apiClient.get(`/rfqs/${id}`);
export const getRfqAttachments = (id) => apiClient.get(`/rfqs/${id}/attachments`);

// --- Quote API ---
export const getQuotesForRfq = (rfqId) => apiClient.get(`/rfqs/${rfqId}/quotes`);
export const createQuote = (rfqId, quoteData) => apiClient.post(`/rfqs/${rfqId}/quotes`, quoteData);
export const acceptQuote = (quoteId) => apiClient.post(`/quotes/${quoteId}/accept`);

// --- Company API ---
export const getCompanyProfile = (id) => apiClient.get(`/companies/${id}`);
export const updateCompanyProfile = (id, data) => apiClient.put(`/companies/${id}`, data);


// --- Order API ---
export const getOrders = () => apiClient.get('/orders');
export const updateOrderStatus = (orderId, status) => {
    return apiClient.patch(`/orders/${orderId}/status`, { status });

};
export const getChatHistory = (rfqId) => apiClient.get(`/rfqs/${rfqId}/messages`);

export const getMyProfile = () => apiClient.get('/users/me');
export const changePassword = (data) => apiClient.put('/users/me/password', data);

export const getBuyerStats = () => apiClient.get('/analytics/buyer-stats');
export const getSupplierStats = () => apiClient.get('/analytics/supplier-stats');
export const getSpendingBySupplier = () => apiClient.get('/analytics/spending-by-supplier');

// --- Capability API ---
export const getAllCapabilities = () => apiClient.get('/capabilities');
export const getCompanyCapabilities = (companyId) => apiClient.get(`/capabilities/company/${companyId}`);
export const addCapabilityToCompany = (capabilityId) => apiClient.post('/capabilities/my-company', { capability_id: capabilityId });
export const removeCapabilityFromCompany = (capabilityId) => apiClient.delete(`/capabilities/my-company/${capabilityId}`);


// --- Payment API ---
export const createCheckoutSession = (orderId) => apiClient.post(`/orders/${orderId}/create-checkout-session`);
// --- Notification API ---
// --- 【新增】Notification API ---
export const getNotifications = () => apiClient.get('/notifications');
export const markNotificationAsRead = (id) => apiClient.put(`/notifications/${id}/read`);
// --- 【新增】Notification API ---
export const markAllNotificationsAsRead = () => apiClient.put('/notifications/read-all');