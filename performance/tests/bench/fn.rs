use super::*;

#[test]
fn benchmark_suite_produces_valid_metrics_and_json() {
    let metrics: Vec<BenchMetric> = vec![
        bench_reactive_signals_and_memos(200),
        bench_selector_o1(200),
        bench_layout_cache(20),
        bench_unicode_glyph_atlas(5),
        bench_software_full_redraw(2),
        bench_software_rasterizer(2),
        bench_hardware_gpu_batching(10),
        bench_game_ecs_physics(5),
        bench_game_particles(20),
        bench_game3d_pbr_bvh_raytrace(1),
        bench_game3d_cinematic_max_rt(1),
        bench_stress_long_list_100k(3),
        bench_stress_complex_layout_2000(3),
        bench_stress_bulk_reactive_1000(2),
        bench_stress_wasm_dynamic_hotswap(2),
        bench_stress_hidpi_4x_1440x3168(2),
        bench_stress_scroll_physics_and_hittest(10),
        bench_software_rounded_clip_cards(2),
        bench_software_primitives_image_vec(2),
        bench_software_portal_overlay_anim(4),
        bench_hardware_active_batching(10),
        bench_font_text_wrap_measure_cache(20),
        bench_app_smooth_scroll_fractional(10),
        bench_ui_markdown_codeblock_mount(2),
        bench_ui_view_macro_mount_500_nodes(2),
        bench_ui_router_freeze_switch(10),
        bench_ui_datatable_filter_select(6),
    ];
    let report: BenchReport = BenchReport::new(metrics);
    assert_eq!(report.get_metrics().len(), 27);
    for m in report.get_metrics().iter() {
        assert!(m.get_iterations() > 0);
        assert!(m.get_ns_per_op() > 0.0);
        assert!(m.get_ops_per_sec() > 0.0);
    }
    let json: String = report_to_json(&report);
    let expected_names: [&str; 27] = [
        "reactive_signal_memo_diamond",
        "reactive_selector_o1_switch",
        "layout_clean_subtree_cache",
        "font_unicode_cjk_atlas_pack",
        "render_software_full_redraw",
        "render_software_sdf_800x600",
        "render_hardware_gpu_batching",
        "game_ecs_physics_256_bodies",
        "game_particles_1024_pool",
        "game3d_pbr_bvh_raytrace",
        "game3d_cinematic_max_rt_125pct",
        "stress_long_list_virtual_100k",
        "stress_complex_layout_2000_nodes",
        "stress_bulk_reactive_1000_nodes",
        "stress_wasm_dynamic_load_hotswap",
        "stress_hidpi_4x_1440x3168_scroll",
        "stress_240hz_scroll_and_hittest",
        "render_software_rounded_clip_cards",
        "render_software_primitives_image_vec",
        "render_software_portal_overlay_anim",
        "render_hardware_active_batching",
        "font_text_wrap_measure_cache",
        "app_smooth_scroll_fractional",
        "ui_markdown_codeblock_mount",
        "ui_view_macro_mount_500_nodes",
        "ui_router_freeze_switch",
        "ui_datatable_filter_select",
    ];
    for expected in expected_names.iter() {
        assert!(json.contains(expected));
    }
    let filtered: BenchReport = run_filtered_benchmark_suite(Some("reactive_signal_memo_diamond"));
    assert_eq!(filtered.get_metrics().len(), 1);
    assert_eq!(
        filtered.get_metrics()[0].get_name().as_str(),
        "reactive_signal_memo_diamond"
    );
}

#[test]
fn round_zero_contracts_behave_as_specified() {
    reset_reactive_runtime();
    let measurer: DeterministicTextMeasurer = DeterministicTextMeasurer;
    let style: TextStyle = TextStyle::default();
    let sample: &str = "Deterministic text measurement 跨端排版";
    let full: TextLayoutResult = measurer.measure(sample, style, Some(180.0));
    let fast_size: Size = measurer.measure_size(sample, style, Some(180.0));
    assert_eq!(fast_size, full.get_size());
    let mut tree: SceneTree = SceneTree::default();
    let mut custom_style: Style = Style::default();
    custom_style.get_mut_paint().set_opacity(0.42);
    let node_id: NodeId = tree.create_view(custom_style);
    let borrowed: &Style = tree.get_node(node_id).unwrap().style_ref();
    assert_eq!(*borrowed, custom_style);
    let sig: Signal<u32> = Signal::new(1);
    assert!(!sig.has_subscribers());
    let (scope, _memo): (Scope, Memo<u32>) = Scope::run(|_: Scope| {
        let derived: Memo<u32> = Memo::new(move || sig.get() * 2);
        let _: u32 = derived.get();
        derived
    });
    assert!(sig.has_subscribers());
    scope.dispose();
    assert!(!sig.has_subscribers());
}
