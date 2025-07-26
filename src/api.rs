// src/api.rs
use actix_web::web;
use crate::handlers::{auth_handler, rfq_handler, quote_handler, order_handler, chat_handler, auth_middleware::Auth, company_handler, user_handler,analytics_handler,};

pub fn config(cfg: &mut web::ServiceConfig) {
    // 公开路由，不需要登录
    cfg.service(
        web::scope("/api/auth")
            .route("/register", web::post().to(auth_handler::register))
            .route("/login", web::post().to(auth_handler::login)),
    );

    // 受保护的RFQ路由
    cfg.service(
        web::scope("/api/rfqs")
            .wrap(Auth)
            .route("", web::post().to(rfq_handler::post_rfq))
            .route("", web::get().to(rfq_handler::get_rfqs))
            .route("/{rfq_id}", web::get().to(rfq_handler::get_rfq_detail))
            .route("/{rfq_id}/attachments", web::get().to(rfq_handler::get_attachments))
            .route("/{rfq_id}/quotes", web::post().to(quote_handler::post_quote))
            .route("/{rfq_id}/quotes", web::get().to(quote_handler::get_quotes)),
    );

    // 受保护的Quote路由
    cfg.service(
        web::scope("/api/quotes")
            .wrap(Auth)
            .route("/{quote_id}/accept", web::post().to(quote_handler::post_accept_quote)),
    );

    // 受保护的Order路由
    cfg.service(
        web::scope("/api/orders")
            .wrap(Auth)
            .route("", web::get().to(order_handler::get_orders))
            .route("/{order_id}/status", web::patch().to(order_handler::patch_order_status)),
    );

    // --- 新增受保护的User路由 ---
    cfg.service(
        web::scope("/api/users")
            .wrap(Auth)
            .route("/me", web::get().to(user_handler::get_me)) // GET /api/users/me
            .route("/me/password", web::put().to(user_handler::update_password)), // PUT /api/users/me/password
    );

    // --- 新增受保护的Company路由 ---
    cfg.service(
        web::scope("/api/companies")
            .wrap(Auth) // 查看和修改都需要登录
            .route("/{company_id}", web::get().to(company_handler::get_profile))
            .route("/{company_id}", web::put().to(company_handler::update_profile)),
    );

    // --- 新增受保护的Analytics路由 ---
    cfg.service(
        web::scope("/api/analytics")
            .wrap(Auth)
            .route("/buyer-stats", web::get().to(analytics_handler::get_stats))
            .route("/spending-by-supplier", web::get().to(analytics_handler::get_spending_breakdown)),
    );

    // --- WebSocket路由 ---
    // 这个路由的认证是在handler内部手动完成的，所以不需要外层的Auth中间件
    cfg.service(
        web::scope("/ws")
            .route("/chat/{rfq_id}", web::get().to(chat_handler::start_chat_session))
    );
}