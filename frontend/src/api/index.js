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

// --- RFQ API ---
export const createRfq = (formData) => { /* ... */ };

// **替换** 原有的 getRfqs 函数
export const getRfqs = (params) => {
    // axios 会自动将 params 对象转换为 URL 查询字符串
    // 例如 { search: 'test', city: '上海' } 会变成 ?search=test&city=上海
    return apiClient.get('/rfqs', { params });
};

export const getRfqById = (id) => apiClient.get(`/rfqs/${id}`);
export const getRfqAttachments = (id) => apiClient.get(`/rfqs/${id}/attachments`); // <-- 新增

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
export const getSpendingBySupplier = () => apiClient.get('/analytics/spending-by-supplier');
