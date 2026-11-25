#include <linux/module.h>
#include <linux/export-internal.h>
#include <linux/compiler.h>

MODULE_INFO(name, KBUILD_MODNAME);

__visible struct module __this_module
__section(".gnu.linkonce.this_module") = {
	.name = KBUILD_MODNAME,
	.init = init_module,
#ifdef CONFIG_MODULE_UNLOAD
	.exit = cleanup_module,
#endif
	.arch = MODULE_ARCH_INIT,
};



static const struct modversion_info ____versions[]
__used __section("__versions") = {
	{ 0x71356ef9, "__kmalloc_noprof" },
	{ 0x22d66ca6, "get_pid_task" },
	{ 0xe28760b4, "register_ftrace_function" },
	{ 0xcb8b6ec6, "kfree" },
	{ 0x7a42e8de, "find_get_pid" },
	{ 0xd272d446, "__fentry__" },
	{ 0x2d2a2ac6, "__put_cred" },
	{ 0xbd03ed67, "__ref_stack_chk_guard" },
	{ 0xd272d446, "__stack_chk_fail" },
	{ 0xff0106da, "refcount_warn_saturate" },
	{ 0x9479a1e8, "strnlen" },
	{ 0xc669d4ce, "access_process_vm" },
	{ 0xd70733be, "sized_strscpy" },
	{ 0xdd77a1de, "ftrace_set_filter_ip" },
	{ 0xe28760b4, "unregister_ftrace_function" },
	{ 0x7b5dad0e, "const_current_task" },
	{ 0x4f1e5fd0, "__list_del_entry_valid_or_report" },
	{ 0xe54e0a6b, "__fortify_panic" },
	{ 0xd272d446, "__x86_return_thunk" },
	{ 0x092a35a2, "_copy_to_user" },
	{ 0xb9fcd065, "call_rcu" },
	{ 0xd49f887b, "kobject_del" },
	{ 0xe5179ac5, "validate_usercopy_range" },
	{ 0xb6377019, "register_kprobe" },
	{ 0x2de0a194, "unregister_kprobe" },
	{ 0x9c74798e, "prepare_kernel_cred" },
	{ 0x1cf09ab5, "__put_task_struct_rcu_cb" },
	{ 0xe541896d, "put_pid" },
	{ 0xa61fd7aa, "__check_object_size" },
	{ 0x092a35a2, "_copy_from_user" },
	{ 0x955c458f, "module_layout" },
};

static const u32 ____version_ext_crcs[]
__used __section("__version_ext_crcs") = {
	0x71356ef9,
	0x22d66ca6,
	0xe28760b4,
	0xcb8b6ec6,
	0x7a42e8de,
	0xd272d446,
	0x2d2a2ac6,
	0xbd03ed67,
	0xd272d446,
	0xff0106da,
	0x9479a1e8,
	0xc669d4ce,
	0xd70733be,
	0xdd77a1de,
	0xe28760b4,
	0x7b5dad0e,
	0x4f1e5fd0,
	0xe54e0a6b,
	0xd272d446,
	0x092a35a2,
	0xb9fcd065,
	0xd49f887b,
	0xe5179ac5,
	0xb6377019,
	0x2de0a194,
	0x9c74798e,
	0x1cf09ab5,
	0xe541896d,
	0xa61fd7aa,
	0x092a35a2,
	0x955c458f,
};
static const char ____version_ext_names[]
__used __section("__version_ext_names") =
	"__kmalloc_noprof\0"
	"get_pid_task\0"
	"register_ftrace_function\0"
	"kfree\0"
	"find_get_pid\0"
	"__fentry__\0"
	"__put_cred\0"
	"__ref_stack_chk_guard\0"
	"__stack_chk_fail\0"
	"refcount_warn_saturate\0"
	"strnlen\0"
	"access_process_vm\0"
	"sized_strscpy\0"
	"ftrace_set_filter_ip\0"
	"unregister_ftrace_function\0"
	"const_current_task\0"
	"__list_del_entry_valid_or_report\0"
	"__fortify_panic\0"
	"__x86_return_thunk\0"
	"_copy_to_user\0"
	"call_rcu\0"
	"kobject_del\0"
	"validate_usercopy_range\0"
	"register_kprobe\0"
	"unregister_kprobe\0"
	"prepare_kernel_cred\0"
	"__put_task_struct_rcu_cb\0"
	"put_pid\0"
	"__check_object_size\0"
	"_copy_from_user\0"
	"module_layout\0"
;

MODULE_INFO(depends, "");


MODULE_INFO(srcversion, "98277A36CA2B895AD84B9E6");

MODULE_INFO(suserelease, "openSUSE Tumbleweed");
