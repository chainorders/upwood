import React from 'react';
import { 
  Popover, 
  PopoverTrigger, 
  PopoverContent, 
  Button,
  ScrollShadow
} from "@nextui-org/react";
import { CircleDot, Bell } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

interface Notification {
  id: number;
  title: string;
  message: string;
  type: 'info' | 'success' | 'warning';
  timestamp: string;
  isRead: boolean;
}

export function NotificationBell() {
  const [isOpen, setIsOpen] = React.useState(false);
  const [notifications, setNotifications] = React.useState<Notification[]>([
    {
      id: 1,
      title: "Investment Successful",
      message: "Your investment in Baltic Pine Forest has been confirmed",
      type: "success",
      timestamp: "2024-03-15T14:30:00",
      isRead: false
    },
    {
      id: 2,
      title: "New Project Available",
      message: "Nordic Spruce Estate is now open for investment",
      type: "info",
      timestamp: "2024-03-15T12:00:00",
      isRead: false
    },
    {
      id: 3,
      title: "KYC Verification Required",
      message: "Please complete your KYC verification to continue investing",
      type: "warning",
      timestamp: "2024-03-14T16:45:00",
      isRead: true
    }
  ]);

  const hasUnread = notifications.some(n => !n.isRead);

  const markAsRead = (notificationId: number, event: React.MouseEvent) => {
    event.stopPropagation(); // Prevent the notification click handler from firing
    setNotifications(prev => 
      prev.map(notification => 
        notification.id === notificationId 
          ? { ...notification, isRead: true }
          : notification
      )
    );
  };

  const markAllAsRead = () => {
    setNotifications(prev => 
      prev.map(notification => ({ ...notification, isRead: true }))
    );
  };

  const handleNotificationClick = (notification: Notification) => {
    // Handle notification click (e.g., navigate to relevant page)
    console.log('Clicked notification:', notification);
    if (!notification.isRead) {
      markAsRead(notification.id, event as React.MouseEvent);
    }
  };

  const formatTimestamp = (timestamp: string) => {
    const date = new Date(timestamp);
    const now = new Date();
    const diffInHours = Math.abs(now.getTime() - date.getTime()) / 36e5;

    if (diffInHours < 24) {
      return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    } else if (diffInHours < 48) {
      return 'Yesterday';
    } else {
      return date.toLocaleDateString();
    }
  };

  return (
    <Popover 
      placement="bottom-end" 
      isOpen={isOpen}
      onOpenChange={setIsOpen}
      offset={12}
      showArrow
    >
      <PopoverTrigger>
        <Button
          className="w-full justify-start text-gray-400 hover:text-white relative"
          variant="light"
          startContent={<Bell className="w-5 h-5" />}
        >
          <div className="flex items-center justify-between w-full">
            <span>Notifications</span>
            {hasUnread && (
              <div className="absolute top-2 right-2 w-2 h-2 bg-red-500 rounded-full" />
            )}
          </div>
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[320px] p-0 bg-gray-900 border border-gray-800">
        <div className="p-3 border-b border-gray-800">
          <div className="flex items-center justify-between">
            <h3 className="text-sm font-medium text-white">Notifications</h3>
            {hasUnread && (
              <Button
                size="sm"
                variant="light"
                className="text-gray-400 hover:text-white text-xs"
                onClick={markAllAsRead}
              >
                Mark all as read
              </Button>
            )}
          </div>
        </div>
        <ScrollShadow className="max-h-[320px]">
          <AnimatePresence initial={false}>
            {notifications.length > 0 ? (
              notifications.map((notification) => (
                <motion.div
                  key={notification.id}
                  initial={{ opacity: 0, y: -10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, height: 0 }}
                  transition={{ duration: 0.2 }}
                  className={`group p-3 border-b border-gray-800 hover:bg-gray-800/50 cursor-pointer relative ${
                    !notification.isRead ? 'bg-gray-800/30' : ''
                  }`}
                  onClick={() => handleNotificationClick(notification)}
                >
                  <div className="flex items-start gap-3">
                    {!notification.isRead && (
                      <CircleDot className="w-2 h-2 text-eco-green flex-shrink-0 mt-1" />
                    )}
                    <div className="flex-1 min-w-0">
                      <h4 className="font-medium text-sm text-white truncate mb-1">
                        {notification.title}
                      </h4>
                      <p className="text-xs text-gray-400 mb-1 line-clamp-2">
                        {notification.message}
                      </p>
                      <div className="flex items-center justify-between">
                        <p className="text-xs text-gray-500">
                          {formatTimestamp(notification.timestamp)}
                        </p>
                        {!notification.isRead && (
                          <Button
                            size="sm"
                            variant="light"
                            className="opacity-0 group-hover:opacity-100 transition-opacity text-gray-400 hover:text-white text-xs h-6 min-w-0"
                            onClick={(e) => markAsRead(notification.id, e)}
                          >
                            Mark as read
                          </Button>
                        )}
                      </div>
                    </div>
                  </div>
                </motion.div>
              ))
            ) : (
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="p-6 text-center text-gray-400"
              >
                <Bell className="w-6 h-6 mx-auto mb-2 text-gray-600" />
                <p className="text-sm">No notifications</p>
              </motion.div>
            )}
          </AnimatePresence>
        </ScrollShadow>
      </PopoverContent>
    </Popover>
  );
}