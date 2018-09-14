#ifndef __RECKLESS_FIXED_H__
#define __RECKLESS_FIXED_H__

#ifndef NO_INCLUDE_WITHIN_HEADERS
#include <gtk/gtk.h>
#endif

#define RECKLESS_FIXED_TYPE                  (reckless_fixed_get_type ())
#define RECKLESS_FIXED(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), RECKLESS_FIXED_TYPE, RecklessFixed))
#define RECKLESS_FIXED_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST  ((klass), RECKLESS_FIXED_TYPE, RecklessFixedClass))
#define IS_RECKLESS_FIXED(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), RECKLESS_FIXED_TYPE))
#define IS_RECKLESS_FIXED_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE  ((klass), RECKLESS_FIXED_TYPE))
#define RECKLESS_FIXED_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS  ((obj), RECKLESS_FIXED_TYPE, RecklessFixedClass))

typedef struct _RecklessFixed      RecklessFixed;
typedef struct _RecklessFixedClass RecklessFixedClass;

struct _RecklessFixed
{
    GtkFixed container;
};

struct _RecklessFixedClass
{
    GtkFixedClass container_class;
};

GType reckless_fixed_get_type(void);
GtkWidget* reckless_fixed_new(void);

static void reckless_fixed_get_preferred_width(GtkWidget *widget, int *minimal, int *natural);
static void reckless_fixed_get_preferred_height(GtkWidget *widget, int *minimal, int *natural);

#endif /* __RECKLESS_FIXED_H__ */
