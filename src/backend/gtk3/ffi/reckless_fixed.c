#include <gtk/gtk.h>
#include "reckless_fixed.h"

G_DEFINE_TYPE( RecklessFixed, reckless_fixed, GTK_TYPE_FIXED )

static void reckless_fixed_class_init( RecklessFixedClass* klass ) {
	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);
	widget_class->get_preferred_width = reckless_fixed_get_preferred_width;
	widget_class->get_preferred_height = reckless_fixed_get_preferred_height;
}
static void reckless_fixed_init( RecklessFixed* self ) {}

static void reckless_fixed_get_preferred_width(GtkWidget *widget, int *minimal, int *natural) {
	*minimal = *natural = 1;
}
static void reckless_fixed_get_preferred_height(GtkWidget *widget, int *minimal, int *natural) {
	*minimal = *natural = 1;
}
GtkWidget* reckless_fixed_new (void) {
  return g_object_new (RECKLESS_FIXED_TYPE, NULL);
}
